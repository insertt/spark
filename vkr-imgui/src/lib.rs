extern crate arrayvec;
extern crate imgui;
extern crate vkr;
#[macro_use]
extern crate memoffset;

use arrayvec::ArrayVec;
use imgui::{FrameSize, ImDrawIdx, ImDrawVert, ImGui, Ui};
use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_uchar, c_void};
use std::slice;
use vkr::vk;
use vkr::{Builder, Device};

fn load_shader_module(device: &Device, bytes: &[u8]) -> vk::ShaderModule {
    let shader_module_create_info = vk::ShaderModuleCreateInfo {
        code_size: bytes.len(),
        p_code: bytes.as_ptr() as *const u32,
        ..Default::default()
    };
    unsafe { device.create_shader_module(&shader_module_create_info, None) }.unwrap()
}

fn get_memory_type_index(
    physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
    type_filter: u32,
    optimal_property_flags: vk::MemoryPropertyFlags,
    fallback_property_flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    for i in 0..physical_device_memory_properties.memory_type_count {
        let mt = &physical_device_memory_properties.memory_types[i as usize];
        if (type_filter & (1 << i)) != 0 && mt.property_flags.contains(optimal_property_flags) {
            return Some(i);
        }
    }
    for i in 0..physical_device_memory_properties.memory_type_count {
        let mt = &physical_device_memory_properties.memory_types[i as usize];
        if (type_filter & (1 << i)) != 0 && mt.property_flags.contains(fallback_property_flags) {
            return Some(i);
        }
    }
    None
}

fn align_up(x: u32, alignment: u32) -> u32 {
    (x + alignment - 1) & !(alignment - 1)
}

pub struct Renderer {
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    vertex_buffers: [vk::Buffer; Renderer::FRAME_COUNT],
    vertex_mem_offsets: [usize; Renderer::FRAME_COUNT],
    index_buffers: [vk::Buffer; Renderer::FRAME_COUNT],
    index_mem_offsets: [usize; Renderer::FRAME_COUNT],
    image_buffer: vk::Buffer,
    host_mem: vk::DeviceMemory,
    host_mapping: *mut c_void,
    image_width: u32,
    image_height: u32,
    image: vk::Image,
    _local_mem: vk::DeviceMemory,
    descriptor_set: vk::DescriptorSet,
    atom_size: u32,
    frame_index: usize,
    image_is_valid: bool,
}

impl Renderer {
    const QUAD_COUNT_PER_FRAME: usize = 64 * 1024;
    const VERTEX_COUNT_PER_FRAME: usize = 4 * Renderer::QUAD_COUNT_PER_FRAME;
    const INDEX_COUNT_PER_FRAME: usize = 6 * Renderer::QUAD_COUNT_PER_FRAME;
    const PUSH_CONSTANT_SIZE: usize = 8;
    const FRAME_COUNT: usize = 2;

    pub fn new(
        device: &Device,
        physical_device_properties: &vk::PhysicalDeviceProperties,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        render_pass: vk::RenderPass,
        imgui: &mut ImGui,
    ) -> Self {
        let vertex_shader = load_shader_module(device, include_bytes!("imgui.vert.spv"));
        let fragment_shader = load_shader_module(device, include_bytes!("imgui.frag.spv"));

        let sampler = {
            let sampler_create_info = vk::SamplerCreateInfo {
                mag_filter: vk::Filter::LINEAR,
                min_filter: vk::Filter::LINEAR,
                ..Default::default()
            };
            unsafe { device.create_sampler(&sampler_create_info, None) }.unwrap()
        };

        let descriptor_set_layout = {
            let binding = vk::DescriptorSetLayoutBinding::builder()
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                .p_immutable_samplers(slice::from_ref(&sampler));
            let descriptor_set_layout_create_info =
                vk::DescriptorSetLayoutCreateInfo::builder().p_bindings(slice::from_ref(&binding));
            unsafe { device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None) }
                .unwrap()
        };

        let pipeline_layout = {
            let push_constant_range = vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::VERTEX,
                offset: 0,
                size: Renderer::PUSH_CONSTANT_SIZE as u32,
            };
            let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::builder()
                .p_set_layouts(slice::from_ref(&descriptor_set_layout))
                .p_push_constant_ranges(slice::from_ref(&push_constant_range));
            unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None) }.unwrap()
        };

        let pipeline = {
            let shader_entry_name = CStr::from_bytes_with_nul(b"main\0").unwrap();
            let shader_stage_create_info = [
                vk::PipelineShaderStageCreateInfo {
                    stage: vk::ShaderStageFlags::VERTEX,
                    module: Some(vertex_shader),
                    p_name: shader_entry_name.as_ptr(),
                    ..Default::default()
                },
                vk::PipelineShaderStageCreateInfo {
                    stage: vk::ShaderStageFlags::FRAGMENT,
                    module: Some(fragment_shader),
                    p_name: shader_entry_name.as_ptr(),
                    ..Default::default()
                },
            ];

            let vertex_input_binding = vk::VertexInputBindingDescription {
                binding: 0,
                stride: mem::size_of::<ImDrawVert>() as u32,
                input_rate: vk::VertexInputRate::VERTEX,
            };
            let vertex_input_attributes = [
                vk::VertexInputAttributeDescription {
                    location: 0,
                    binding: 0,
                    format: vk::Format::R32G32_SFLOAT,
                    offset: offset_of!(ImDrawVert, pos) as u32,
                },
                vk::VertexInputAttributeDescription {
                    location: 1,
                    binding: 0,
                    format: vk::Format::R32G32_SFLOAT,
                    offset: offset_of!(ImDrawVert, uv) as u32,
                },
                vk::VertexInputAttributeDescription {
                    location: 2,
                    binding: 0,
                    format: vk::Format::R8G8B8A8_UNORM,
                    offset: offset_of!(ImDrawVert, col) as u32,
                },
            ];

            let vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo::builder()
                .p_vertex_binding_descriptions(slice::from_ref(&vertex_input_binding))
                .p_vertex_attribute_descriptions(&vertex_input_attributes);

            let input_assembly_state_create_info = vk::PipelineInputAssemblyStateCreateInfo {
                topology: vk::PrimitiveTopology::TRIANGLE_LIST,
                ..Default::default()
            };

            let viewport_state_create_info = vk::PipelineViewportStateCreateInfo {
                viewport_count: 1,
                scissor_count: 1,
                ..Default::default()
            };

            let rasterization_state_create_info = vk::PipelineRasterizationStateCreateInfo {
                polygon_mode: vk::PolygonMode::FILL,
                cull_mode: vk::CullModeFlags::BACK,
                front_face: vk::FrontFace::CLOCKWISE,
                line_width: 1.0,
                ..Default::default()
            };
            let multisample_state_create_info = vk::PipelineMultisampleStateCreateInfo {
                rasterization_samples: vk::SampleCountFlags::N1,
                ..Default::default()
            };

            let color_blend_attachment_state = vk::PipelineColorBlendAttachmentState {
                blend_enable: vk::TRUE,
                src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
                dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
                color_blend_op: vk::BlendOp::ADD,
                src_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
                dst_alpha_blend_factor: vk::BlendFactor::ZERO,
                alpha_blend_op: vk::BlendOp::ADD,
                color_write_mask: vk::ColorComponentFlags::all(),
            };
            let color_blend_state_create_info = vk::PipelineColorBlendStateCreateInfo::builder()
                .p_attachments(slice::from_ref(&color_blend_attachment_state));

            let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let pipeline_dynamic_state_create_info =
                vk::PipelineDynamicStateCreateInfo::builder().p_dynamic_states(&dynamic_states);

            let pipeline_create_info = vk::GraphicsPipelineCreateInfo::builder()
                .p_stages(&shader_stage_create_info)
                .p_vertex_input_state(Some(&vertex_input_state_create_info))
                .p_input_assembly_state(Some(&input_assembly_state_create_info))
                .p_viewport_state(Some(&viewport_state_create_info))
                .p_rasterization_state(&rasterization_state_create_info)
                .p_multisample_state(Some(&multisample_state_create_info))
                .p_color_blend_state(Some(&color_blend_state_create_info))
                .p_dynamic_state(Some(&pipeline_dynamic_state_create_info))
                .layout(pipeline_layout)
                .render_pass(render_pass);

            unsafe {
                device.create_graphics_pipelines_single(
                    None,
                    slice::from_ref(&pipeline_create_info),
                    None,
                )
            }.unwrap()
        };

        unsafe { device.destroy_shader_module(Some(vertex_shader), None) };
        unsafe { device.destroy_shader_module(Some(fragment_shader), None) };

        let mut host_allocation_size = 0;
        let mut host_memory_type_filter = 0xffffffff;

        let (vertex_buffers, vertex_mem_offsets) = {
            let buffer_create_info = vk::BufferCreateInfo {
                size: (Renderer::VERTEX_COUNT_PER_FRAME * mem::size_of::<ImDrawVert>())
                    as vk::DeviceSize,
                usage: vk::BufferUsageFlags::VERTEX_BUFFER,
                ..Default::default()
            };
            let mut buffers = ArrayVec::<[vk::Buffer; Renderer::FRAME_COUNT]>::new();
            let mut mem_offsets = ArrayVec::<[usize; Renderer::FRAME_COUNT]>::new();
            for _i in 0..Renderer::FRAME_COUNT {
                let buffer = unsafe { device.create_buffer(&buffer_create_info, None) }.unwrap();
                let mem_req = unsafe { device.get_buffer_memory_requirements(buffer) };
                assert_eq!(mem_req.size, buffer_create_info.size);
                let mem_offset = host_allocation_size as usize;
                host_allocation_size += buffer_create_info.size;
                buffers.push(buffer);
                mem_offsets.push(mem_offset);
                host_memory_type_filter &= mem_req.memory_type_bits;
            }
            (
                buffers.into_inner().unwrap(),
                mem_offsets.into_inner().unwrap(),
            )
        };

        let (index_buffers, index_mem_offsets) = {
            let buffer_create_info = vk::BufferCreateInfo {
                size: (Renderer::INDEX_COUNT_PER_FRAME * mem::size_of::<ImDrawIdx>())
                    as vk::DeviceSize,
                usage: vk::BufferUsageFlags::INDEX_BUFFER,
                ..Default::default()
            };
            let mut buffers = ArrayVec::<[vk::Buffer; Renderer::FRAME_COUNT]>::new();
            let mut mem_offsets = ArrayVec::<[usize; Renderer::FRAME_COUNT]>::new();
            for _i in 0..Renderer::FRAME_COUNT {
                let buffer = unsafe { device.create_buffer(&buffer_create_info, None) }.unwrap();
                let mem_req = unsafe { device.get_buffer_memory_requirements(buffer) };
                assert_eq!(mem_req.size, buffer_create_info.size);
                let mem_offset = host_allocation_size as usize;
                host_allocation_size += buffer_create_info.size;
                buffers.push(buffer);
                mem_offsets.push(mem_offset);
                host_memory_type_filter &= mem_req.memory_type_bits;
            }
            (
                buffers.into_inner().unwrap(),
                mem_offsets.into_inner().unwrap(),
            )
        };

        let (image_width, image_height) =
            imgui.prepare_texture(|handle| (handle.width, handle.height));

        let (image_buffer, image_mem_offset) = {
            let buffer_create_info = vk::BufferCreateInfo {
                size: (image_width * image_height * 4) as vk::DeviceSize,
                usage: vk::BufferUsageFlags::TRANSFER_SRC,
                ..Default::default()
            };
            let buffer = unsafe { device.create_buffer(&buffer_create_info, None) }.unwrap();
            let mem_req = unsafe { device.get_buffer_memory_requirements(buffer) };
            assert_eq!(mem_req.size, buffer_create_info.size);
            let mem_offset = host_allocation_size as usize;
            host_allocation_size += buffer_create_info.size;
            host_memory_type_filter &= mem_req.memory_type_bits;
            (buffer, mem_offset)
        };

        let host_mem = {
            let memory_type_index = get_memory_type_index(
                physical_device_memory_properties,
                host_memory_type_filter,
                vk::MemoryPropertyFlags::DEVICE_LOCAL
                    | vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ).unwrap();
            let memory_allocate_info = vk::MemoryAllocateInfo {
                allocation_size: host_allocation_size,
                memory_type_index,
                ..Default::default()
            };
            unsafe { device.allocate_memory(&memory_allocate_info, None) }.unwrap()
        };

        for (&buf, &ofs) in vertex_buffers.iter().zip(vertex_mem_offsets.iter()) {
            unsafe { device.bind_buffer_memory(buf, host_mem, ofs as vk::DeviceSize) }.unwrap();
        }
        for (&buf, &ofs) in index_buffers.iter().zip(index_mem_offsets.iter()) {
            unsafe { device.bind_buffer_memory(buf, host_mem, ofs as vk::DeviceSize) }.unwrap();
        }
        unsafe {
            device.bind_buffer_memory(image_buffer, host_mem, image_mem_offset as vk::DeviceSize)
        }.unwrap();

        let host_mapping =
            unsafe { device.map_memory(host_mem, 0, vk::WHOLE_SIZE, Default::default()) }.unwrap();

        let image = {
            let image_create_info = vk::ImageCreateInfo {
                image_type: vk::ImageType::N2D,
                format: vk::Format::R8G8B8A8_UNORM,
                extent: vk::Extent3D {
                    width: image_width,
                    height: image_height,
                    depth: 1,
                },
                mip_levels: 1,
                array_layers: 1,
                samples: vk::SampleCountFlags::N1,
                usage: vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
                ..Default::default()
            };
            unsafe { device.create_image(&image_create_info, None) }.unwrap()
        };

        let (local_allocation_size, local_memory_type_filter) = {
            let mem_req = unsafe { device.get_image_memory_requirements(image) };
            (mem_req.size, mem_req.memory_type_bits)
        };

        let local_mem = {
            let memory_type_index = get_memory_type_index(
                physical_device_memory_properties,
                local_memory_type_filter,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            ).unwrap();
            let memory_allocate_info = vk::MemoryAllocateInfo {
                allocation_size: local_allocation_size,
                memory_type_index,
                ..Default::default()
            };
            unsafe { device.allocate_memory(&memory_allocate_info, None) }.unwrap()
        };

        unsafe { device.bind_image_memory(image, local_mem, 0) }.unwrap();

        let descriptor_pool = {
            let descriptor_pool_sizes = [vk::DescriptorPoolSize {
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
            }];
            let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo::builder()
                .max_sets(1)
                .p_pool_sizes(&descriptor_pool_sizes);
            unsafe { device.create_descriptor_pool(&descriptor_pool_create_info, None) }.unwrap()
        };

        let descriptor_set = {
            let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(descriptor_pool)
                .p_set_layouts(slice::from_ref(&descriptor_set_layout));
            unsafe { device.allocate_descriptor_sets_single(&descriptor_set_allocate_info) }
                .unwrap()
        };

        let image_view = {
            let image_view_create_info = vk::ImageViewCreateInfo {
                image: Some(image),
                view_type: vk::ImageViewType::N2D,
                format: vk::Format::R8G8B8A8_UNORM,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };
            unsafe { device.create_image_view(&image_view_create_info, None) }.unwrap()
        };

        {
            let image_info = vk::DescriptorImageInfo {
                sampler: Some(sampler),
                image_view: Some(image_view),
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            };
            let write_descriptor_set = vk::WriteDescriptorSet::builder()
                .dst_set(descriptor_set)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .p_image_info(slice::from_ref(&image_info));
            unsafe { device.update_descriptor_sets(slice::from_ref(&write_descriptor_set), &[]) };
        }

        let atom_size = physical_device_properties.limits.non_coherent_atom_size as u32;

        imgui.prepare_texture(|handle| {
            let image_base =
                unsafe { (host_mapping as *mut u8).add(image_mem_offset) } as *mut c_uchar;
            assert_eq!(
                (image_width * image_height * 4) as usize,
                handle.pixels.len()
            );
            unsafe {
                image_base.copy_from_nonoverlapping(handle.pixels.as_ptr(), handle.pixels.len())
            };

            let mapped_memory_range = vk::MappedMemoryRange {
                memory: Some(host_mem),
                offset: image_mem_offset as vk::DeviceSize,
                size: align_up((image_width * image_height * 4) as u32, atom_size)
                    as vk::DeviceSize,
                ..Default::default()
            };
            unsafe { device.flush_mapped_memory_ranges(slice::from_ref(&mapped_memory_range)) }
                .unwrap();
        });

        imgui.prepare_texture(|_handle| false);

        Self {
            pipeline_layout,
            pipeline,
            vertex_buffers,
            vertex_mem_offsets,
            index_buffers,
            index_mem_offsets,
            image_buffer,
            host_mem,
            host_mapping,
            image_width,
            image_height,
            image,
            _local_mem: local_mem,
            descriptor_set,
            atom_size,
            frame_index: 0,
            image_is_valid: false,
        }
    }

    pub fn begin_frame(&mut self, device: &Device, command_buffer: vk::CommandBuffer) {
        if !self.image_is_valid {
            let transfer_from_undef = vk::ImageMemoryBarrier {
                dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                image: Some(self.image),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };
            unsafe {
                device.cmd_pipeline_barrier(
                    command_buffer,
                    vk::PipelineStageFlags::HOST,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    slice::from_ref(&transfer_from_undef),
                )
            };

            let buffer_image_copy = vk::BufferImageCopy {
                image_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    layer_count: 1,
                    ..Default::default()
                },
                image_extent: vk::Extent3D {
                    width: self.image_width,
                    height: self.image_height,
                    depth: 1,
                },
                ..Default::default()
            };
            unsafe {
                device.cmd_copy_buffer_to_image(
                    command_buffer,
                    self.image_buffer,
                    self.image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    slice::from_ref(&buffer_image_copy),
                )
            };

            let shader_from_transfer = vk::ImageMemoryBarrier {
                src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                dst_access_mask: vk::AccessFlags::SHADER_READ,
                old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                image: Some(self.image),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };
            unsafe {
                device.cmd_pipeline_barrier(
                    command_buffer,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    slice::from_ref(&shader_from_transfer),
                )
            };

            self.image_is_valid = true;
        }
    }

    pub fn render(&mut self, ui: Ui, device: &Device, command_buffer: vk::CommandBuffer) {
        let FrameSize {
            logical_size: (width, height),
            ..
        } = ui.frame_size();
        let width = width as f32;
        let height = height as f32;

        ui.render(|ui, mut draw_data| -> Result<_, u32> {
            draw_data.scale_clip_rects(ui.imgui().display_framebuffer_scale());

            let vertex_buffer = self.vertex_buffers[self.frame_index];
            let vertex_mem_offset = self.vertex_mem_offsets[self.frame_index];
            let index_buffer = self.index_buffers[self.frame_index];
            let index_mem_offset = self.index_mem_offsets[self.frame_index];

            unsafe {
                device.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline,
                );
                device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline_layout,
                    0,
                    slice::from_ref(&self.descriptor_set),
                    &[],
                );
            }

            let dims_rcp = [1.0 / width, 1.0 / height];
            unsafe {
                device.cmd_push_constants(
                    command_buffer,
                    self.pipeline_layout,
                    vk::ShaderStageFlags::VERTEX,
                    0,
                    Renderer::PUSH_CONSTANT_SIZE as u32,
                    dims_rcp.as_ptr() as *const _,
                )
            };

            let viewport = vk::Viewport {
                width,
                height,
                max_depth: 1.0,
                ..Default::default()
            };
            unsafe { device.cmd_set_viewport(command_buffer, 0, slice::from_ref(&viewport)) };

            unsafe {
                device.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    slice::from_ref(&vertex_buffer),
                    &[0],
                );
                device.cmd_bind_index_buffer(
                    command_buffer,
                    index_buffer,
                    0,
                    vk::IndexType::UINT16,
                );
            }

            let vertex_base =
                unsafe { (self.host_mapping as *mut u8).add(vertex_mem_offset) } as *mut ImDrawVert;
            let index_base =
                unsafe { (self.host_mapping as *mut u8).add(index_mem_offset) } as *mut ImDrawIdx;
            let mut vertex_offset = 0;
            let mut index_offset = 0;
            for draw_list in &draw_data {
                let next_vertex_offset = vertex_offset + draw_list.vtx_buffer.len();
                let next_index_offset = index_offset + draw_list.idx_buffer.len();
                if next_vertex_offset > Renderer::VERTEX_COUNT_PER_FRAME
                    || next_index_offset > Renderer::INDEX_COUNT_PER_FRAME
                {
                    break;
                }

                unsafe {
                    vertex_base.add(vertex_offset).copy_from_nonoverlapping(
                        draw_list.vtx_buffer.as_ptr(),
                        draw_list.vtx_buffer.len(),
                    );
                    index_base.add(index_offset).copy_from_nonoverlapping(
                        draw_list.idx_buffer.as_ptr(),
                        draw_list.idx_buffer.len(),
                    );
                }

                for cmd in draw_list.cmd_buffer {
                    let scissor = vk::Rect2D {
                        offset: vk::Offset2D {
                            x: cmd.clip_rect.x as i32,
                            y: cmd.clip_rect.y as i32,
                        },
                        extent: vk::Extent2D {
                            width: (cmd.clip_rect.z - cmd.clip_rect.x) as u32,
                            height: (cmd.clip_rect.w - cmd.clip_rect.y) as u32,
                        },
                    };
                    unsafe {
                        device.cmd_set_scissor(command_buffer, 0, slice::from_ref(&scissor));
                        device.cmd_draw_indexed(
                            command_buffer,
                            cmd.elem_count,
                            1,
                            index_offset as u32,
                            vertex_offset as i32,
                            0,
                        );
                    }

                    index_offset += cmd.elem_count as usize;
                }

                vertex_offset = next_vertex_offset;
                assert_eq!(index_offset, next_index_offset);
            }

            let mapped_ranges = [
                vk::MappedMemoryRange {
                    memory: Some(self.host_mem),
                    offset: vertex_mem_offset as vk::DeviceSize,
                    size: align_up(
                        (vertex_offset * mem::size_of::<ImDrawVert>()) as u32,
                        self.atom_size,
                    ) as vk::DeviceSize,
                    ..Default::default()
                },
                vk::MappedMemoryRange {
                    memory: Some(self.host_mem),
                    offset: index_mem_offset as vk::DeviceSize,
                    size: align_up(
                        (index_offset * mem::size_of::<ImDrawIdx>()) as u32,
                        self.atom_size,
                    ) as vk::DeviceSize,
                    ..Default::default()
                },
            ];
            unsafe { device.flush_mapped_memory_ranges(&mapped_ranges) }.unwrap();

            Ok(())
        }).unwrap();

        self.frame_index = (1 + self.frame_index) % Renderer::FRAME_COUNT;
    }
}