#![allow(clippy::wrong_self_convention)]

use super::vk;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

pub trait Builder<'a> {
    type Type;
    fn builder() -> Self::Type;
}

impl Builder<'_> for vk::BaseOutStructure {
    type Type = BaseOutStructureBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BaseOutStructureBuilder {
    inner: vk::BaseOutStructure,
}
impl BaseOutStructureBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut vk::BaseOutStructure) -> Self {
        self.inner.p_next = p_next;
        self
    }
}
impl Deref for BaseOutStructureBuilder {
    type Target = vk::BaseOutStructure;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BaseInStructure {
    type Type = BaseInStructureBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BaseInStructureBuilder<'a> {
    inner: vk::BaseInStructure,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BaseInStructureBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: Option<&'a vk::BaseInStructure>) -> Self {
        self.inner.p_next = p_next.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for BaseInStructureBuilder<'a> {
    type Target = vk::BaseInStructure;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ApplicationInfo {
    type Type = ApplicationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ApplicationInfoBuilder<'a> {
    inner: vk::ApplicationInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ApplicationInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_application_name(mut self, p_application_name: Option<&'a CStr>) -> Self {
        self.inner.p_application_name = p_application_name.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn application_version(mut self, application_version: u32) -> Self {
        self.inner.application_version = application_version;
        self
    }
    pub fn p_engine_name(mut self, p_engine_name: Option<&'a CStr>) -> Self {
        self.inner.p_engine_name = p_engine_name.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn engine_version(mut self, engine_version: u32) -> Self {
        self.inner.engine_version = engine_version;
        self
    }
    pub fn api_version(mut self, api_version: vk::Version) -> Self {
        self.inner.api_version = api_version;
        self
    }
}
impl<'a> Deref for ApplicationInfoBuilder<'a> {
    type Target = vk::ApplicationInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AllocationCallbacks {
    type Type = AllocationCallbacksBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AllocationCallbacksBuilder {
    inner: vk::AllocationCallbacks,
}
impl AllocationCallbacksBuilder {
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
    pub fn pfn_allocation(mut self, pfn_allocation: vk::FnAllocationFunction) -> Self {
        self.inner.pfn_allocation = Some(pfn_allocation);
        self
    }
    pub fn pfn_reallocation(mut self, pfn_reallocation: vk::FnReallocationFunction) -> Self {
        self.inner.pfn_reallocation = Some(pfn_reallocation);
        self
    }
    pub fn pfn_free(mut self, pfn_free: vk::FnFreeFunction) -> Self {
        self.inner.pfn_free = Some(pfn_free);
        self
    }
    pub fn pfn_internal_allocation(
        mut self,
        pfn_internal_allocation: Option<vk::FnInternalAllocationNotification>,
    ) -> Self {
        self.inner.pfn_internal_allocation = pfn_internal_allocation;
        self
    }
    pub fn pfn_internal_free(mut self, pfn_internal_free: Option<vk::FnInternalFreeNotification>) -> Self {
        self.inner.pfn_internal_free = pfn_internal_free;
        self
    }
}
impl Deref for AllocationCallbacksBuilder {
    type Target = vk::AllocationCallbacks;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceQueueCreateInfo {
    type Type = DeviceQueueCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceQueueCreateInfoBuilder<'a> {
    inner: vk::DeviceQueueCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceQueueCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceQueueCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
    pub fn p_queue_priorities(mut self, p_queue_priorities: &'a [f32]) -> Self {
        self.inner.queue_count = p_queue_priorities.len() as u32;
        self.inner.p_queue_priorities = p_queue_priorities.as_ptr();
        self
    }
}
impl<'a> Deref for DeviceQueueCreateInfoBuilder<'a> {
    type Target = vk::DeviceQueueCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceCreateInfo {
    type Type = DeviceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceCreateInfoBuilder<'a> {
    inner: vk::DeviceCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_queue_create_infos(mut self, p_queue_create_infos: &'a [vk::DeviceQueueCreateInfo]) -> Self {
        self.inner.queue_create_info_count = p_queue_create_infos.len() as u32;
        self.inner.p_queue_create_infos = p_queue_create_infos.as_ptr();
        self
    }
    pub fn pp_enabled_layer_names(mut self, pp_enabled_layer_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_layer_count = pp_enabled_layer_names.len() as u32;
        self.inner.pp_enabled_layer_names = pp_enabled_layer_names.as_ptr();
        self
    }
    pub fn pp_enabled_extension_names(mut self, pp_enabled_extension_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_extension_count = pp_enabled_extension_names.len() as u32;
        self.inner.pp_enabled_extension_names = pp_enabled_extension_names.as_ptr();
        self
    }
    pub fn p_enabled_features(mut self, p_enabled_features: Option<&'a vk::PhysicalDeviceFeatures>) -> Self {
        self.inner.p_enabled_features = p_enabled_features.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for DeviceCreateInfoBuilder<'a> {
    type Target = vk::DeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::InstanceCreateInfo {
    type Type = InstanceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct InstanceCreateInfoBuilder<'a> {
    inner: vk::InstanceCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> InstanceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::InstanceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_application_info(mut self, p_application_info: Option<&'a vk::ApplicationInfo>) -> Self {
        self.inner.p_application_info = p_application_info.map_or(ptr::null(), |p| p);
        self
    }
    pub fn pp_enabled_layer_names(mut self, pp_enabled_layer_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_layer_count = pp_enabled_layer_names.len() as u32;
        self.inner.pp_enabled_layer_names = pp_enabled_layer_names.as_ptr();
        self
    }
    pub fn pp_enabled_extension_names(mut self, pp_enabled_extension_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_extension_count = pp_enabled_extension_names.len() as u32;
        self.inner.pp_enabled_extension_names = pp_enabled_extension_names.as_ptr();
        self
    }
}
impl<'a> Deref for InstanceCreateInfoBuilder<'a> {
    type Target = vk::InstanceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryAllocateInfo {
    type Type = MemoryAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryAllocateInfoBuilder {
    inner: vk::MemoryAllocateInfo,
}
impl MemoryAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn allocation_size(mut self, allocation_size: vk::DeviceSize) -> Self {
        self.inner.allocation_size = allocation_size;
        self
    }
    pub fn memory_type_index(mut self, memory_type_index: u32) -> Self {
        self.inner.memory_type_index = memory_type_index;
        self
    }
}
impl Deref for MemoryAllocateInfoBuilder {
    type Target = vk::MemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MappedMemoryRange {
    type Type = MappedMemoryRangeBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MappedMemoryRangeBuilder {
    inner: vk::MappedMemoryRange,
}
impl MappedMemoryRangeBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl Deref for MappedMemoryRangeBuilder {
    type Target = vk::MappedMemoryRange;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSet {
    type Type = WriteDescriptorSetBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetBuilder<'a> {
    inner: vk::WriteDescriptorSet,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> WriteDescriptorSetBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dst_set(mut self, dst_set: vk::DescriptorSet) -> Self {
        self.inner.dst_set = Some(dst_set);
        self
    }
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dst_binding = dst_binding;
        self
    }
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dst_array_element = dst_array_element;
        self
    }
    pub fn p_image_info(mut self, p_image_info: &'a [vk::DescriptorImageInfo]) -> Self {
        self.inner.descriptor_count = p_image_info.len() as u32;
        self.inner.p_image_info = p_image_info.as_ptr();
        self
    }
    pub fn p_buffer_info(mut self, p_buffer_info: &'a [vk::DescriptorBufferInfo]) -> Self {
        self.inner.descriptor_count = p_buffer_info.len() as u32;
        self.inner.p_buffer_info = p_buffer_info.as_ptr();
        self
    }
    pub fn p_texel_buffer_view(mut self, p_texel_buffer_view: &'a [vk::BufferView]) -> Self {
        self.inner.descriptor_count = p_texel_buffer_view.len() as u32;
        self.inner.p_texel_buffer_view = p_texel_buffer_view.as_ptr();
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
}
impl<'a> Deref for WriteDescriptorSetBuilder<'a> {
    type Target = vk::WriteDescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyDescriptorSet {
    type Type = CopyDescriptorSetBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyDescriptorSetBuilder {
    inner: vk::CopyDescriptorSet,
}
impl CopyDescriptorSetBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_set(mut self, src_set: vk::DescriptorSet) -> Self {
        self.inner.src_set = Some(src_set);
        self
    }
    pub fn src_binding(mut self, src_binding: u32) -> Self {
        self.inner.src_binding = src_binding;
        self
    }
    pub fn src_array_element(mut self, src_array_element: u32) -> Self {
        self.inner.src_array_element = src_array_element;
        self
    }
    pub fn dst_set(mut self, dst_set: vk::DescriptorSet) -> Self {
        self.inner.dst_set = Some(dst_set);
        self
    }
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dst_binding = dst_binding;
        self
    }
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dst_array_element = dst_array_element;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
}
impl Deref for CopyDescriptorSetBuilder {
    type Target = vk::CopyDescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferCreateInfo {
    type Type = BufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCreateInfoBuilder<'a> {
    inner: vk::BufferCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BufferCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::BufferCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn usage(mut self, usage: vk::BufferUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
}
impl<'a> Deref for BufferCreateInfoBuilder<'a> {
    type Target = vk::BufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferViewCreateInfo {
    type Type = BufferViewCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferViewCreateInfoBuilder {
    inner: vk::BufferViewCreateInfo,
}
impl BufferViewCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::BufferViewCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn range(mut self, range: vk::DeviceSize) -> Self {
        self.inner.range = range;
        self
    }
}
impl Deref for BufferViewCreateInfoBuilder {
    type Target = vk::BufferViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryBarrier {
    type Type = MemoryBarrierBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryBarrierBuilder {
    inner: vk::MemoryBarrier,
}
impl MemoryBarrierBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
}
impl Deref for MemoryBarrierBuilder {
    type Target = vk::MemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferMemoryBarrier {
    type Type = BufferMemoryBarrierBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferMemoryBarrierBuilder {
    inner: vk::BufferMemoryBarrier,
}
impl BufferMemoryBarrierBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl Deref for BufferMemoryBarrierBuilder {
    type Target = vk::BufferMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageMemoryBarrier {
    type Type = ImageMemoryBarrierBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageMemoryBarrierBuilder {
    inner: vk::ImageMemoryBarrier,
}
impl ImageMemoryBarrierBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn old_layout(mut self, old_layout: vk::ImageLayout) -> Self {
        self.inner.old_layout = old_layout;
        self
    }
    pub fn new_layout(mut self, new_layout: vk::ImageLayout) -> Self {
        self.inner.new_layout = new_layout;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn subresource_range(mut self, subresource_range: vk::ImageSubresourceRange) -> Self {
        self.inner.subresource_range = subresource_range;
        self
    }
}
impl Deref for ImageMemoryBarrierBuilder {
    type Target = vk::ImageMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageCreateInfo {
    type Type = ImageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageCreateInfoBuilder<'a> {
    inner: vk::ImageCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image_type(mut self, image_type: vk::ImageType) -> Self {
        self.inner.image_type = image_type;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn extent(mut self, extent: vk::Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.inner.mip_levels = mip_levels;
        self
    }
    pub fn array_layers(mut self, array_layers: u32) -> Self {
        self.inner.array_layers = array_layers;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
    pub fn initial_layout(mut self, initial_layout: vk::ImageLayout) -> Self {
        self.inner.initial_layout = initial_layout;
        self
    }
}
impl<'a> Deref for ImageCreateInfoBuilder<'a> {
    type Target = vk::ImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageViewCreateInfo {
    type Type = ImageViewCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewCreateInfoBuilder {
    inner: vk::ImageViewCreateInfo,
}
impl ImageViewCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageViewCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn view_type(mut self, view_type: vk::ImageViewType) -> Self {
        self.inner.view_type = view_type;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn components(mut self, components: vk::ComponentMapping) -> Self {
        self.inner.components = components;
        self
    }
    pub fn subresource_range(mut self, subresource_range: vk::ImageSubresourceRange) -> Self {
        self.inner.subresource_range = subresource_range;
        self
    }
}
impl Deref for ImageViewCreateInfoBuilder {
    type Target = vk::ImageViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SparseBufferMemoryBindInfo {
    type Type = SparseBufferMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseBufferMemoryBindInfoBuilder<'a> {
    inner: vk::SparseBufferMemoryBindInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SparseBufferMemoryBindInfoBuilder<'a> {
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.as_ptr();
        self
    }
}
impl<'a> Deref for SparseBufferMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseBufferMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SparseImageOpaqueMemoryBindInfo {
    type Type = SparseImageOpaqueMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    inner: vk::SparseImageOpaqueMemoryBindInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.as_ptr();
        self
    }
}
impl<'a> Deref for SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseImageOpaqueMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SparseImageMemoryBindInfo {
    type Type = SparseImageMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseImageMemoryBindInfoBuilder<'a> {
    inner: vk::SparseImageMemoryBindInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SparseImageMemoryBindInfoBuilder<'a> {
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseImageMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.as_ptr();
        self
    }
}
impl<'a> Deref for SparseImageMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseImageMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindSparseInfo {
    type Type = BindSparseInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindSparseInfoBuilder<'a> {
    inner: vk::BindSparseInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindSparseInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphores(mut self, p_wait_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        self.inner.p_wait_semaphores = p_wait_semaphores.as_ptr();
        self
    }
    pub fn p_buffer_binds(mut self, p_buffer_binds: &'a [vk::SparseBufferMemoryBindInfo]) -> Self {
        self.inner.buffer_bind_count = p_buffer_binds.len() as u32;
        self.inner.p_buffer_binds = p_buffer_binds.as_ptr();
        self
    }
    pub fn p_image_opaque_binds(mut self, p_image_opaque_binds: &'a [vk::SparseImageOpaqueMemoryBindInfo]) -> Self {
        self.inner.image_opaque_bind_count = p_image_opaque_binds.len() as u32;
        self.inner.p_image_opaque_binds = p_image_opaque_binds.as_ptr();
        self
    }
    pub fn p_image_binds(mut self, p_image_binds: &'a [vk::SparseImageMemoryBindInfo]) -> Self {
        self.inner.image_bind_count = p_image_binds.len() as u32;
        self.inner.p_image_binds = p_image_binds.as_ptr();
        self
    }
    pub fn p_signal_semaphores(mut self, p_signal_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphores.len() as u32;
        self.inner.p_signal_semaphores = p_signal_semaphores.as_ptr();
        self
    }
}
impl<'a> Deref for BindSparseInfoBuilder<'a> {
    type Target = vk::BindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ShaderModuleCreateInfo {
    type Type = ShaderModuleCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ShaderModuleCreateInfoBuilder {
    inner: vk::ShaderModuleCreateInfo,
}
impl ShaderModuleCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ShaderModuleCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn code_size(mut self, code_size: usize) -> Self {
        self.inner.code_size = code_size;
        self
    }
    pub fn p_code(mut self, p_code: *const u32) -> Self {
        self.inner.p_code = p_code;
        self
    }
}
impl Deref for ShaderModuleCreateInfoBuilder {
    type Target = vk::ShaderModuleCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutBinding {
    type Type = DescriptorSetLayoutBindingBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutBindingBuilder<'a> {
    inner: vk::DescriptorSetLayoutBinding,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetLayoutBindingBuilder<'a> {
    pub fn binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
    pub fn p_immutable_samplers(mut self, p_immutable_samplers: &'a [vk::Sampler]) -> Self {
        self.inner.descriptor_count = p_immutable_samplers.len() as u32;
        self.inner.p_immutable_samplers = p_immutable_samplers.as_ptr();
        self
    }
    pub fn stage_flags(mut self, stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.stage_flags = stage_flags;
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutBindingBuilder<'a> {
    type Target = vk::DescriptorSetLayoutBinding;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutCreateInfo {
    type Type = DescriptorSetLayoutCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutCreateInfoBuilder<'a> {
    inner: vk::DescriptorSetLayoutCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetLayoutCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorSetLayoutCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_bindings(mut self, p_bindings: &'a [vk::DescriptorSetLayoutBinding]) -> Self {
        self.inner.binding_count = p_bindings.len() as u32;
        self.inner.p_bindings = p_bindings.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutCreateInfoBuilder<'a> {
    type Target = vk::DescriptorSetLayoutCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorPoolCreateInfo {
    type Type = DescriptorPoolCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorPoolCreateInfoBuilder<'a> {
    inner: vk::DescriptorPoolCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorPoolCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn max_sets(mut self, max_sets: u32) -> Self {
        self.inner.max_sets = max_sets;
        self
    }
    pub fn p_pool_sizes(mut self, p_pool_sizes: &'a [vk::DescriptorPoolSize]) -> Self {
        self.inner.pool_size_count = p_pool_sizes.len() as u32;
        self.inner.p_pool_sizes = p_pool_sizes.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorPoolCreateInfoBuilder<'a> {
    type Target = vk::DescriptorPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorSetAllocateInfo {
    type Type = DescriptorSetAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetAllocateInfoBuilder<'a> {
    inner: vk::DescriptorSetAllocateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn descriptor_pool(mut self, descriptor_pool: vk::DescriptorPool) -> Self {
        self.inner.descriptor_pool = Some(descriptor_pool);
        self
    }
    pub fn p_set_layouts(mut self, p_set_layouts: &'a [vk::DescriptorSetLayout]) -> Self {
        self.inner.descriptor_set_count = p_set_layouts.len() as u32;
        self.inner.p_set_layouts = p_set_layouts.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorSetAllocateInfoBuilder<'a> {
    type Target = vk::DescriptorSetAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SpecializationInfo {
    type Type = SpecializationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SpecializationInfoBuilder<'a> {
    inner: vk::SpecializationInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SpecializationInfoBuilder<'a> {
    pub fn p_map_entries(mut self, p_map_entries: &'a [vk::SpecializationMapEntry]) -> Self {
        self.inner.map_entry_count = p_map_entries.len() as u32;
        self.inner.p_map_entries = p_map_entries.as_ptr();
        self
    }
    pub fn p_data<T>(mut self, p_data: &'a [T]) -> Self {
        self.inner.data_size = mem::size_of_val(p_data) as usize;
        self.inner.p_data = p_data.as_ptr() as *const _;
        self
    }
}
impl<'a> Deref for SpecializationInfoBuilder<'a> {
    type Target = vk::SpecializationInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineShaderStageCreateInfo {
    type Type = PipelineShaderStageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineShaderStageCreateInfoBuilder<'a> {
    inner: vk::PipelineShaderStageCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineShaderStageCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineShaderStageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage(mut self, stage: vk::ShaderStageFlags) -> Self {
        self.inner.stage = stage;
        self
    }
    pub fn module(mut self, module: vk::ShaderModule) -> Self {
        self.inner.module = Some(module);
        self
    }
    pub fn p_name(mut self, p_name: &'a CStr) -> Self {
        self.inner.p_name = p_name.as_ptr();
        self
    }
    pub fn p_specialization_info(mut self, p_specialization_info: Option<&'a vk::SpecializationInfo>) -> Self {
        self.inner.p_specialization_info = p_specialization_info.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for PipelineShaderStageCreateInfoBuilder<'a> {
    type Target = vk::PipelineShaderStageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ComputePipelineCreateInfo {
    type Type = ComputePipelineCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ComputePipelineCreateInfoBuilder {
    inner: vk::ComputePipelineCreateInfo,
}
impl ComputePipelineCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage(mut self, stage: vk::PipelineShaderStageCreateInfo) -> Self {
        self.inner.stage = stage;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl Deref for ComputePipelineCreateInfoBuilder {
    type Target = vk::ComputePipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineVertexInputStateCreateInfo {
    type Type = PipelineVertexInputStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineVertexInputStateCreateInfoBuilder<'a> {
    inner: vk::PipelineVertexInputStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineVertexInputStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineVertexInputStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_vertex_binding_descriptions(
        mut self,
        p_vertex_binding_descriptions: &'a [vk::VertexInputBindingDescription],
    ) -> Self {
        self.inner.vertex_binding_description_count = p_vertex_binding_descriptions.len() as u32;
        self.inner.p_vertex_binding_descriptions = p_vertex_binding_descriptions.as_ptr();
        self
    }
    pub fn p_vertex_attribute_descriptions(
        mut self,
        p_vertex_attribute_descriptions: &'a [vk::VertexInputAttributeDescription],
    ) -> Self {
        self.inner.vertex_attribute_description_count = p_vertex_attribute_descriptions.len() as u32;
        self.inner.p_vertex_attribute_descriptions = p_vertex_attribute_descriptions.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineVertexInputStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineVertexInputStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineInputAssemblyStateCreateInfo {
    type Type = PipelineInputAssemblyStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineInputAssemblyStateCreateInfoBuilder {
    inner: vk::PipelineInputAssemblyStateCreateInfo,
}
impl PipelineInputAssemblyStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineInputAssemblyStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn topology(mut self, topology: vk::PrimitiveTopology) -> Self {
        self.inner.topology = topology;
        self
    }
    pub fn primitive_restart_enable(mut self, primitive_restart_enable: bool) -> Self {
        self.inner.primitive_restart_enable = if primitive_restart_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PipelineInputAssemblyStateCreateInfoBuilder {
    type Target = vk::PipelineInputAssemblyStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineTessellationStateCreateInfo {
    type Type = PipelineTessellationStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineTessellationStateCreateInfoBuilder {
    inner: vk::PipelineTessellationStateCreateInfo,
}
impl PipelineTessellationStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineTessellationStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn patch_control_points(mut self, patch_control_points: u32) -> Self {
        self.inner.patch_control_points = patch_control_points;
        self
    }
}
impl Deref for PipelineTessellationStateCreateInfoBuilder {
    type Target = vk::PipelineTessellationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportStateCreateInfo {
    type Type = PipelineViewportStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportStateCreateInfoBuilder<'a> {
    inner: vk::PipelineViewportStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineViewportStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.inner.viewport_count = viewport_count;
        self
    }
    pub fn p_viewports(mut self, p_viewports: &'a [vk::Viewport]) -> Self {
        self.inner.viewport_count = p_viewports.len() as u32;
        self.inner.p_viewports = p_viewports.as_ptr();
        self
    }
    pub fn scissor_count(mut self, scissor_count: u32) -> Self {
        self.inner.scissor_count = scissor_count;
        self
    }
    pub fn p_scissors(mut self, p_scissors: &'a [vk::Rect2D]) -> Self {
        self.inner.scissor_count = p_scissors.len() as u32;
        self.inner.p_scissors = p_scissors.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineViewportStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineRasterizationStateCreateInfo {
    type Type = PipelineRasterizationStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationStateCreateInfoBuilder {
    inner: vk::PipelineRasterizationStateCreateInfo,
}
impl PipelineRasterizationStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_clamp_enable(mut self, depth_clamp_enable: bool) -> Self {
        self.inner.depth_clamp_enable = if depth_clamp_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn rasterizer_discard_enable(mut self, rasterizer_discard_enable: bool) -> Self {
        self.inner.rasterizer_discard_enable = if rasterizer_discard_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn polygon_mode(mut self, polygon_mode: vk::PolygonMode) -> Self {
        self.inner.polygon_mode = polygon_mode;
        self
    }
    pub fn cull_mode(mut self, cull_mode: vk::CullModeFlags) -> Self {
        self.inner.cull_mode = cull_mode;
        self
    }
    pub fn front_face(mut self, front_face: vk::FrontFace) -> Self {
        self.inner.front_face = front_face;
        self
    }
    pub fn depth_bias_enable(mut self, depth_bias_enable: bool) -> Self {
        self.inner.depth_bias_enable = if depth_bias_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
        self.inner.depth_bias_constant_factor = depth_bias_constant_factor;
        self
    }
    pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.inner.depth_bias_clamp = depth_bias_clamp;
        self
    }
    pub fn depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
        self.inner.depth_bias_slope_factor = depth_bias_slope_factor;
        self
    }
    pub fn line_width(mut self, line_width: f32) -> Self {
        self.inner.line_width = line_width;
        self
    }
}
impl Deref for PipelineRasterizationStateCreateInfoBuilder {
    type Target = vk::PipelineRasterizationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineMultisampleStateCreateInfo {
    type Type = PipelineMultisampleStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineMultisampleStateCreateInfoBuilder {
    inner: vk::PipelineMultisampleStateCreateInfo,
}
impl PipelineMultisampleStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineMultisampleStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn rasterization_samples(mut self, rasterization_samples: vk::SampleCountFlags) -> Self {
        self.inner.rasterization_samples = rasterization_samples;
        self
    }
    pub fn sample_shading_enable(mut self, sample_shading_enable: bool) -> Self {
        self.inner.sample_shading_enable = if sample_shading_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn min_sample_shading(mut self, min_sample_shading: f32) -> Self {
        self.inner.min_sample_shading = min_sample_shading;
        self
    }
    pub fn p_sample_mask(mut self, p_sample_mask: *const vk::SampleMask) -> Self {
        self.inner.p_sample_mask = p_sample_mask;
        self
    }
    pub fn alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.inner.alpha_to_coverage_enable = if alpha_to_coverage_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn alpha_to_one_enable(mut self, alpha_to_one_enable: bool) -> Self {
        self.inner.alpha_to_one_enable = if alpha_to_one_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PipelineMultisampleStateCreateInfoBuilder {
    type Target = vk::PipelineMultisampleStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineColorBlendStateCreateInfo {
    type Type = PipelineColorBlendStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineColorBlendStateCreateInfoBuilder<'a> {
    inner: vk::PipelineColorBlendStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineColorBlendStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineColorBlendStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn logic_op_enable(mut self, logic_op_enable: bool) -> Self {
        self.inner.logic_op_enable = if logic_op_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn logic_op(mut self, logic_op: vk::LogicOp) -> Self {
        self.inner.logic_op = logic_op;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::PipelineColorBlendAttachmentState]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineColorBlendStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineColorBlendStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineDynamicStateCreateInfo {
    type Type = PipelineDynamicStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDynamicStateCreateInfoBuilder<'a> {
    inner: vk::PipelineDynamicStateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineDynamicStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDynamicStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_dynamic_states(mut self, p_dynamic_states: &'a [vk::DynamicState]) -> Self {
        self.inner.dynamic_state_count = p_dynamic_states.len() as u32;
        self.inner.p_dynamic_states = p_dynamic_states.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineDynamicStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineDynamicStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineDepthStencilStateCreateInfo {
    type Type = PipelineDepthStencilStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDepthStencilStateCreateInfoBuilder {
    inner: vk::PipelineDepthStencilStateCreateInfo,
}
impl PipelineDepthStencilStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDepthStencilStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_test_enable(mut self, depth_test_enable: bool) -> Self {
        self.inner.depth_test_enable = if depth_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_write_enable(mut self, depth_write_enable: bool) -> Self {
        self.inner.depth_write_enable = if depth_write_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_compare_op(mut self, depth_compare_op: vk::CompareOp) -> Self {
        self.inner.depth_compare_op = depth_compare_op;
        self
    }
    pub fn depth_bounds_test_enable(mut self, depth_bounds_test_enable: bool) -> Self {
        self.inner.depth_bounds_test_enable = if depth_bounds_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stencil_test_enable(mut self, stencil_test_enable: bool) -> Self {
        self.inner.stencil_test_enable = if stencil_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn front(mut self, front: vk::StencilOpState) -> Self {
        self.inner.front = front;
        self
    }
    pub fn back(mut self, back: vk::StencilOpState) -> Self {
        self.inner.back = back;
        self
    }
    pub fn min_depth_bounds(mut self, min_depth_bounds: f32) -> Self {
        self.inner.min_depth_bounds = min_depth_bounds;
        self
    }
    pub fn max_depth_bounds(mut self, max_depth_bounds: f32) -> Self {
        self.inner.max_depth_bounds = max_depth_bounds;
        self
    }
}
impl Deref for PipelineDepthStencilStateCreateInfoBuilder {
    type Target = vk::PipelineDepthStencilStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GraphicsPipelineCreateInfo {
    type Type = GraphicsPipelineCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsPipelineCreateInfoBuilder<'a> {
    inner: vk::GraphicsPipelineCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> GraphicsPipelineCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_vertex_input_state(
        mut self,
        p_vertex_input_state: Option<&'a vk::PipelineVertexInputStateCreateInfo>,
    ) -> Self {
        self.inner.p_vertex_input_state = p_vertex_input_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_input_assembly_state(
        mut self,
        p_input_assembly_state: Option<&'a vk::PipelineInputAssemblyStateCreateInfo>,
    ) -> Self {
        self.inner.p_input_assembly_state = p_input_assembly_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_tessellation_state(
        mut self,
        p_tessellation_state: Option<&'a vk::PipelineTessellationStateCreateInfo>,
    ) -> Self {
        self.inner.p_tessellation_state = p_tessellation_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_viewport_state(mut self, p_viewport_state: Option<&'a vk::PipelineViewportStateCreateInfo>) -> Self {
        self.inner.p_viewport_state = p_viewport_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_rasterization_state(
        mut self,
        p_rasterization_state: &'a vk::PipelineRasterizationStateCreateInfo,
    ) -> Self {
        self.inner.p_rasterization_state = p_rasterization_state;
        self
    }
    pub fn p_multisample_state(
        mut self,
        p_multisample_state: Option<&'a vk::PipelineMultisampleStateCreateInfo>,
    ) -> Self {
        self.inner.p_multisample_state = p_multisample_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_depth_stencil_state(
        mut self,
        p_depth_stencil_state: Option<&'a vk::PipelineDepthStencilStateCreateInfo>,
    ) -> Self {
        self.inner.p_depth_stencil_state = p_depth_stencil_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_color_blend_state(
        mut self,
        p_color_blend_state: Option<&'a vk::PipelineColorBlendStateCreateInfo>,
    ) -> Self {
        self.inner.p_color_blend_state = p_color_blend_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_dynamic_state(mut self, p_dynamic_state: Option<&'a vk::PipelineDynamicStateCreateInfo>) -> Self {
        self.inner.p_dynamic_state = p_dynamic_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = Some(render_pass);
        self
    }
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.inner.subpass = subpass;
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for GraphicsPipelineCreateInfoBuilder<'a> {
    type Target = vk::GraphicsPipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineCacheCreateInfo {
    type Type = PipelineCacheCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCacheCreateInfoBuilder<'a> {
    inner: vk::PipelineCacheCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineCacheCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCacheCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_initial_data<T>(mut self, p_initial_data: &'a [T]) -> Self {
        self.inner.initial_data_size = mem::size_of_val(p_initial_data) as usize;
        self.inner.p_initial_data = p_initial_data.as_ptr() as *const _;
        self
    }
}
impl<'a> Deref for PipelineCacheCreateInfoBuilder<'a> {
    type Target = vk::PipelineCacheCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineLayoutCreateInfo {
    type Type = PipelineLayoutCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineLayoutCreateInfoBuilder<'a> {
    inner: vk::PipelineLayoutCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineLayoutCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineLayoutCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_set_layouts(mut self, p_set_layouts: &'a [vk::DescriptorSetLayout]) -> Self {
        self.inner.set_layout_count = p_set_layouts.len() as u32;
        self.inner.p_set_layouts = p_set_layouts.as_ptr();
        self
    }
    pub fn p_push_constant_ranges(mut self, p_push_constant_ranges: &'a [vk::PushConstantRange]) -> Self {
        self.inner.push_constant_range_count = p_push_constant_ranges.len() as u32;
        self.inner.p_push_constant_ranges = p_push_constant_ranges.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineLayoutCreateInfoBuilder<'a> {
    type Target = vk::PipelineLayoutCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SamplerCreateInfo {
    type Type = SamplerCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerCreateInfoBuilder {
    inner: vk::SamplerCreateInfo,
}
impl SamplerCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SamplerCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn mag_filter(mut self, mag_filter: vk::Filter) -> Self {
        self.inner.mag_filter = mag_filter;
        self
    }
    pub fn min_filter(mut self, min_filter: vk::Filter) -> Self {
        self.inner.min_filter = min_filter;
        self
    }
    pub fn mipmap_mode(mut self, mipmap_mode: vk::SamplerMipmapMode) -> Self {
        self.inner.mipmap_mode = mipmap_mode;
        self
    }
    pub fn address_mode_u(mut self, address_mode_u: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_u = address_mode_u;
        self
    }
    pub fn address_mode_v(mut self, address_mode_v: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_v = address_mode_v;
        self
    }
    pub fn address_mode_w(mut self, address_mode_w: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_w = address_mode_w;
        self
    }
    pub fn mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.inner.mip_lod_bias = mip_lod_bias;
        self
    }
    pub fn anisotropy_enable(mut self, anisotropy_enable: bool) -> Self {
        self.inner.anisotropy_enable = if anisotropy_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn max_anisotropy(mut self, max_anisotropy: f32) -> Self {
        self.inner.max_anisotropy = max_anisotropy;
        self
    }
    pub fn compare_enable(mut self, compare_enable: bool) -> Self {
        self.inner.compare_enable = if compare_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn compare_op(mut self, compare_op: vk::CompareOp) -> Self {
        self.inner.compare_op = compare_op;
        self
    }
    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.inner.min_lod = min_lod;
        self
    }
    pub fn max_lod(mut self, max_lod: f32) -> Self {
        self.inner.max_lod = max_lod;
        self
    }
    pub fn border_color(mut self, border_color: vk::BorderColor) -> Self {
        self.inner.border_color = border_color;
        self
    }
    pub fn unnormalized_coordinates(mut self, unnormalized_coordinates: bool) -> Self {
        self.inner.unnormalized_coordinates = if unnormalized_coordinates { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for SamplerCreateInfoBuilder {
    type Target = vk::SamplerCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CommandPoolCreateInfo {
    type Type = CommandPoolCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandPoolCreateInfoBuilder {
    inner: vk::CommandPoolCreateInfo,
}
impl CommandPoolCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::CommandPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
}
impl Deref for CommandPoolCreateInfoBuilder {
    type Target = vk::CommandPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CommandBufferAllocateInfo {
    type Type = CommandBufferAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferAllocateInfoBuilder {
    inner: vk::CommandBufferAllocateInfo,
}
impl CommandBufferAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn command_pool(mut self, command_pool: vk::CommandPool) -> Self {
        self.inner.command_pool = Some(command_pool);
        self
    }
    pub fn level(mut self, level: vk::CommandBufferLevel) -> Self {
        self.inner.level = level;
        self
    }
    pub fn command_buffer_count(mut self, command_buffer_count: u32) -> Self {
        self.inner.command_buffer_count = command_buffer_count;
        self
    }
}
impl Deref for CommandBufferAllocateInfoBuilder {
    type Target = vk::CommandBufferAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CommandBufferInheritanceInfo {
    type Type = CommandBufferInheritanceInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceInfoBuilder {
    inner: vk::CommandBufferInheritanceInfo,
}
impl CommandBufferInheritanceInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn render_pass(mut self, render_pass: Option<vk::RenderPass>) -> Self {
        self.inner.render_pass = render_pass;
        self
    }
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.inner.subpass = subpass;
        self
    }
    pub fn framebuffer(mut self, framebuffer: Option<vk::Framebuffer>) -> Self {
        self.inner.framebuffer = framebuffer;
        self
    }
    pub fn occlusion_query_enable(mut self, occlusion_query_enable: bool) -> Self {
        self.inner.occlusion_query_enable = if occlusion_query_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn query_flags(mut self, query_flags: vk::QueryControlFlags) -> Self {
        self.inner.query_flags = query_flags;
        self
    }
    pub fn pipeline_statistics(mut self, pipeline_statistics: vk::QueryPipelineStatisticFlags) -> Self {
        self.inner.pipeline_statistics = pipeline_statistics;
        self
    }
}
impl Deref for CommandBufferInheritanceInfoBuilder {
    type Target = vk::CommandBufferInheritanceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandBufferBeginInfo {
    type Type = CommandBufferBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferBeginInfoBuilder<'a> {
    inner: vk::CommandBufferBeginInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CommandBufferBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::CommandBufferUsageFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_inheritance_info(mut self, p_inheritance_info: Option<&'a vk::CommandBufferInheritanceInfo>) -> Self {
        self.inner.p_inheritance_info = p_inheritance_info.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for CommandBufferBeginInfoBuilder<'a> {
    type Target = vk::CommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassBeginInfo {
    type Type = RenderPassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassBeginInfoBuilder<'a> {
    inner: vk::RenderPassBeginInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = Some(render_pass);
        self
    }
    pub fn framebuffer(mut self, framebuffer: vk::Framebuffer) -> Self {
        self.inner.framebuffer = Some(framebuffer);
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
    pub fn p_clear_values(mut self, p_clear_values: &'a [vk::ClearValue]) -> Self {
        self.inner.clear_value_count = p_clear_values.len() as u32;
        self.inner.p_clear_values = p_clear_values.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassBeginInfoBuilder<'a> {
    type Target = vk::RenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassDescription {
    type Type = SubpassDescriptionBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDescriptionBuilder<'a> {
    inner: vk::SubpassDescription,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassDescriptionBuilder<'a> {
    pub fn flags(mut self, flags: vk::SubpassDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn p_input_attachments(mut self, p_input_attachments: &'a [vk::AttachmentReference]) -> Self {
        self.inner.input_attachment_count = p_input_attachments.len() as u32;
        self.inner.p_input_attachments = p_input_attachments.as_ptr();
        self
    }
    pub fn p_color_attachments(
        mut self,
        p_color_attachments: &'a [vk::AttachmentReference],
        p_resolve_attachments: Option<&'a [vk::AttachmentReference]>,
    ) -> Self {
        self.inner.color_attachment_count = p_color_attachments.len() as u32;
        if let Some(s) = p_resolve_attachments {
            assert_eq!(self.inner.color_attachment_count, s.len() as u32);
        }
        self.inner.p_color_attachments = p_color_attachments.as_ptr();
        self.inner.p_resolve_attachments = p_resolve_attachments.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn p_depth_stencil_attachment(
        mut self,
        p_depth_stencil_attachment: Option<&'a vk::AttachmentReference>,
    ) -> Self {
        self.inner.p_depth_stencil_attachment = p_depth_stencil_attachment.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_preserve_attachments(mut self, p_preserve_attachments: &'a [u32]) -> Self {
        self.inner.preserve_attachment_count = p_preserve_attachments.len() as u32;
        self.inner.p_preserve_attachments = p_preserve_attachments.as_ptr();
        self
    }
}
impl<'a> Deref for SubpassDescriptionBuilder<'a> {
    type Target = vk::SubpassDescription;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassCreateInfo {
    type Type = RenderPassCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassCreateInfoBuilder<'a> {
    inner: vk::RenderPassCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderPassCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::AttachmentDescription]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
    pub fn p_subpasses(mut self, p_subpasses: &'a [vk::SubpassDescription]) -> Self {
        self.inner.subpass_count = p_subpasses.len() as u32;
        self.inner.p_subpasses = p_subpasses.as_ptr();
        self
    }
    pub fn p_dependencies(mut self, p_dependencies: &'a [vk::SubpassDependency]) -> Self {
        self.inner.dependency_count = p_dependencies.len() as u32;
        self.inner.p_dependencies = p_dependencies.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassCreateInfoBuilder<'a> {
    type Target = vk::RenderPassCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::EventCreateInfo {
    type Type = EventCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct EventCreateInfoBuilder {
    inner: vk::EventCreateInfo,
}
impl EventCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::EventCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for EventCreateInfoBuilder {
    type Target = vk::EventCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::FenceCreateInfo {
    type Type = FenceCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FenceCreateInfoBuilder {
    inner: vk::FenceCreateInfo,
}
impl FenceCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::FenceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for FenceCreateInfoBuilder {
    type Target = vk::FenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreCreateInfo {
    type Type = SemaphoreCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreCreateInfoBuilder {
    inner: vk::SemaphoreCreateInfo,
}
impl SemaphoreCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for SemaphoreCreateInfoBuilder {
    type Target = vk::SemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::QueryPoolCreateInfo {
    type Type = QueryPoolCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueryPoolCreateInfoBuilder {
    inner: vk::QueryPoolCreateInfo,
}
impl QueryPoolCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::QueryPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn query_type(mut self, query_type: vk::QueryType) -> Self {
        self.inner.query_type = query_type;
        self
    }
    pub fn query_count(mut self, query_count: u32) -> Self {
        self.inner.query_count = query_count;
        self
    }
    pub fn pipeline_statistics(mut self, pipeline_statistics: vk::QueryPipelineStatisticFlags) -> Self {
        self.inner.pipeline_statistics = pipeline_statistics;
        self
    }
}
impl Deref for QueryPoolCreateInfoBuilder {
    type Target = vk::QueryPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FramebufferCreateInfo {
    type Type = FramebufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FramebufferCreateInfoBuilder<'a> {
    inner: vk::FramebufferCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FramebufferCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::FramebufferCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = Some(render_pass);
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::ImageView]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }
    pub fn layers(mut self, layers: u32) -> Self {
        self.inner.layers = layers;
        self
    }
}
impl<'a> Deref for FramebufferCreateInfoBuilder<'a> {
    type Target = vk::FramebufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubmitInfo {
    type Type = SubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubmitInfoBuilder<'a> {
    inner: vk::SubmitInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphores(
        mut self,
        p_wait_semaphores: &'a [vk::Semaphore],
        p_wait_dst_stage_mask: &'a [vk::PipelineStageFlags],
    ) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        assert_eq!(self.inner.wait_semaphore_count, p_wait_dst_stage_mask.len() as u32);
        self.inner.p_wait_semaphores = p_wait_semaphores.as_ptr();
        self.inner.p_wait_dst_stage_mask = p_wait_dst_stage_mask.as_ptr();
        self
    }
    pub fn p_command_buffers(mut self, p_command_buffers: &'a [vk::CommandBuffer]) -> Self {
        self.inner.command_buffer_count = p_command_buffers.len() as u32;
        self.inner.p_command_buffers = p_command_buffers.as_ptr();
        self
    }
    pub fn p_signal_semaphores(mut self, p_signal_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphores.len() as u32;
        self.inner.p_signal_semaphores = p_signal_semaphores.as_ptr();
        self
    }
}
impl<'a> Deref for SubmitInfoBuilder<'a> {
    type Target = vk::SubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayModeCreateInfoKHR {
    type Type = DisplayModeCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayModeCreateInfoKHRBuilder {
    inner: vk::DisplayModeCreateInfoKHR,
}
impl DisplayModeCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DisplayModeCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn parameters(mut self, parameters: vk::DisplayModeParametersKHR) -> Self {
        self.inner.parameters = parameters;
        self
    }
}
impl Deref for DisplayModeCreateInfoKHRBuilder {
    type Target = vk::DisplayModeCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplaySurfaceCreateInfoKHR {
    type Type = DisplaySurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplaySurfaceCreateInfoKHRBuilder {
    inner: vk::DisplaySurfaceCreateInfoKHR,
}
impl DisplaySurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DisplaySurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn display_mode(mut self, display_mode: vk::DisplayModeKHR) -> Self {
        self.inner.display_mode = Some(display_mode);
        self
    }
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.inner.plane_index = plane_index;
        self
    }
    pub fn plane_stack_index(mut self, plane_stack_index: u32) -> Self {
        self.inner.plane_stack_index = plane_stack_index;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
    pub fn global_alpha(mut self, global_alpha: f32) -> Self {
        self.inner.global_alpha = global_alpha;
        self
    }
    pub fn alpha_mode(mut self, alpha_mode: vk::DisplayPlaneAlphaFlagsKHR) -> Self {
        self.inner.alpha_mode = alpha_mode;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent2D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
}
impl Deref for DisplaySurfaceCreateInfoKHRBuilder {
    type Target = vk::DisplaySurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayPresentInfoKHR {
    type Type = DisplayPresentInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPresentInfoKHRBuilder {
    inner: vk::DisplayPresentInfoKHR,
}
impl DisplayPresentInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_rect(mut self, src_rect: vk::Rect2D) -> Self {
        self.inner.src_rect = src_rect;
        self
    }
    pub fn dst_rect(mut self, dst_rect: vk::Rect2D) -> Self {
        self.inner.dst_rect = dst_rect;
        self
    }
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.inner.persistent = if persistent { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for DisplayPresentInfoKHRBuilder {
    type Target = vk::DisplayPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AndroidSurfaceCreateInfoKHR {
    type Type = AndroidSurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AndroidSurfaceCreateInfoKHRBuilder {
    inner: vk::AndroidSurfaceCreateInfoKHR,
}
impl AndroidSurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AndroidSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn window(mut self, window: *mut vk::ANativeWindow) -> Self {
        self.inner.window = window;
        self
    }
}
impl Deref for AndroidSurfaceCreateInfoKHRBuilder {
    type Target = vk::AndroidSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ViSurfaceCreateInfoNN {
    type Type = ViSurfaceCreateInfoNNBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ViSurfaceCreateInfoNNBuilder {
    inner: vk::ViSurfaceCreateInfoNN,
}
impl ViSurfaceCreateInfoNNBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ViSurfaceCreateFlagsNN) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn window(mut self, window: *mut c_void) -> Self {
        self.inner.window = window;
        self
    }
}
impl Deref for ViSurfaceCreateInfoNNBuilder {
    type Target = vk::ViSurfaceCreateInfoNN;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::WaylandSurfaceCreateInfoKHR {
    type Type = WaylandSurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WaylandSurfaceCreateInfoKHRBuilder {
    inner: vk::WaylandSurfaceCreateInfoKHR,
}
impl WaylandSurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::WaylandSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn display(mut self, display: *mut vk::wl_display) -> Self {
        self.inner.display = display;
        self
    }
    pub fn surface(mut self, surface: *mut vk::wl_surface) -> Self {
        self.inner.surface = surface;
        self
    }
}
impl Deref for WaylandSurfaceCreateInfoKHRBuilder {
    type Target = vk::WaylandSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::Win32SurfaceCreateInfoKHR {
    type Type = Win32SurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32SurfaceCreateInfoKHRBuilder {
    inner: vk::Win32SurfaceCreateInfoKHR,
}
impl Win32SurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::Win32SurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn hinstance(mut self, hinstance: vk::HINSTANCE) -> Self {
        self.inner.hinstance = hinstance;
        self
    }
    pub fn hwnd(mut self, hwnd: vk::HWND) -> Self {
        self.inner.hwnd = hwnd;
        self
    }
}
impl Deref for Win32SurfaceCreateInfoKHRBuilder {
    type Target = vk::Win32SurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::XlibSurfaceCreateInfoKHR {
    type Type = XlibSurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct XlibSurfaceCreateInfoKHRBuilder {
    inner: vk::XlibSurfaceCreateInfoKHR,
}
impl XlibSurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::XlibSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn dpy(mut self, dpy: *mut vk::Display) -> Self {
        self.inner.dpy = dpy;
        self
    }
    pub fn window(mut self, window: vk::Window) -> Self {
        self.inner.window = window;
        self
    }
}
impl Deref for XlibSurfaceCreateInfoKHRBuilder {
    type Target = vk::XlibSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::XcbSurfaceCreateInfoKHR {
    type Type = XcbSurfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct XcbSurfaceCreateInfoKHRBuilder {
    inner: vk::XcbSurfaceCreateInfoKHR,
}
impl XcbSurfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::XcbSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn connection(mut self, connection: *mut vk::xcb_connection_t) -> Self {
        self.inner.connection = connection;
        self
    }
    pub fn window(mut self, window: vk::xcb_window_t) -> Self {
        self.inner.window = window;
        self
    }
}
impl Deref for XcbSurfaceCreateInfoKHRBuilder {
    type Target = vk::XcbSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DirectFBSurfaceCreateInfoEXT {
    type Type = DirectFBSurfaceCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DirectFBSurfaceCreateInfoEXTBuilder {
    inner: vk::DirectFBSurfaceCreateInfoEXT,
}
impl DirectFBSurfaceCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DirectFBSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn dfb(mut self, dfb: *mut vk::IDirectFB) -> Self {
        self.inner.dfb = dfb;
        self
    }
    pub fn surface(mut self, surface: *mut vk::IDirectFBSurface) -> Self {
        self.inner.surface = surface;
        self
    }
}
impl Deref for DirectFBSurfaceCreateInfoEXTBuilder {
    type Target = vk::DirectFBSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImagePipeSurfaceCreateInfoFUCHSIA {
    type Type = ImagePipeSurfaceCreateInfoFUCHSIABuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIABuilder {
    inner: vk::ImagePipeSurfaceCreateInfoFUCHSIA,
}
impl ImagePipeSurfaceCreateInfoFUCHSIABuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImagePipeSurfaceCreateFlagsFUCHSIA) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image_pipe_handle(mut self, image_pipe_handle: vk::zx_handle_t) -> Self {
        self.inner.image_pipe_handle = image_pipe_handle;
        self
    }
}
impl Deref for ImagePipeSurfaceCreateInfoFUCHSIABuilder {
    type Target = vk::ImagePipeSurfaceCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SwapchainCreateInfoKHR {
    type Type = SwapchainCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainCreateInfoKHRBuilder<'a> {
    inner: vk::SwapchainCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SwapchainCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SwapchainCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn surface(mut self, surface: vk::SurfaceKHR) -> Self {
        self.inner.surface = Some(surface);
        self
    }
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.inner.min_image_count = min_image_count;
        self
    }
    pub fn image_format(mut self, image_format: vk::Format) -> Self {
        self.inner.image_format = image_format;
        self
    }
    pub fn image_color_space(mut self, image_color_space: vk::ColorSpaceKHR) -> Self {
        self.inner.image_color_space = image_color_space;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent2D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
    pub fn image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.inner.image_array_layers = image_array_layers;
        self
    }
    pub fn image_usage(mut self, image_usage: vk::ImageUsageFlags) -> Self {
        self.inner.image_usage = image_usage;
        self
    }
    pub fn image_sharing_mode(mut self, image_sharing_mode: vk::SharingMode) -> Self {
        self.inner.image_sharing_mode = image_sharing_mode;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
    pub fn pre_transform(mut self, pre_transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.pre_transform = pre_transform;
        self
    }
    pub fn composite_alpha(mut self, composite_alpha: vk::CompositeAlphaFlagsKHR) -> Self {
        self.inner.composite_alpha = composite_alpha;
        self
    }
    pub fn present_mode(mut self, present_mode: vk::PresentModeKHR) -> Self {
        self.inner.present_mode = present_mode;
        self
    }
    pub fn clipped(mut self, clipped: bool) -> Self {
        self.inner.clipped = if clipped { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn old_swapchain(mut self, old_swapchain: Option<vk::SwapchainKHR>) -> Self {
        self.inner.old_swapchain = old_swapchain;
        self
    }
}
impl<'a> Deref for SwapchainCreateInfoKHRBuilder<'a> {
    type Target = vk::SwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PresentInfoKHR {
    type Type = PresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentInfoKHRBuilder<'a> {
    inner: vk::PresentInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphores(mut self, p_wait_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        self.inner.p_wait_semaphores = p_wait_semaphores.as_ptr();
        self
    }
    pub fn p_swapchains(mut self, p_swapchains: &'a [vk::SwapchainKHR], p_image_indices: &'a [u32]) -> Self {
        self.inner.swapchain_count = p_swapchains.len() as u32;
        assert_eq!(self.inner.swapchain_count, p_image_indices.len() as u32);
        self.inner.p_swapchains = p_swapchains.as_ptr();
        self.inner.p_image_indices = p_image_indices.as_ptr();
        self
    }
    pub fn p_results(mut self, p_results: *mut vk::Result) -> Self {
        self.inner.p_results = p_results;
        self
    }
}
impl<'a> Deref for PresentInfoKHRBuilder<'a> {
    type Target = vk::PresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DebugReportCallbackCreateInfoEXT {
    type Type = DebugReportCallbackCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugReportCallbackCreateInfoEXTBuilder {
    inner: vk::DebugReportCallbackCreateInfoEXT,
}
impl DebugReportCallbackCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugReportFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pfn_callback(mut self, pfn_callback: vk::FnDebugReportCallbackEXT) -> Self {
        self.inner.pfn_callback = Some(pfn_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl Deref for DebugReportCallbackCreateInfoEXTBuilder {
    type Target = vk::DebugReportCallbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ValidationFlagsEXT {
    type Type = ValidationFlagsEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationFlagsEXTBuilder<'a> {
    inner: vk::ValidationFlagsEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ValidationFlagsEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_disabled_validation_checks(mut self, p_disabled_validation_checks: &'a [vk::ValidationCheckEXT]) -> Self {
        self.inner.disabled_validation_check_count = p_disabled_validation_checks.len() as u32;
        self.inner.p_disabled_validation_checks = p_disabled_validation_checks.as_ptr();
        self
    }
}
impl<'a> Deref for ValidationFlagsEXTBuilder<'a> {
    type Target = vk::ValidationFlagsEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ValidationFeaturesEXT {
    type Type = ValidationFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationFeaturesEXTBuilder<'a> {
    inner: vk::ValidationFeaturesEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ValidationFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_enabled_validation_features(
        mut self,
        p_enabled_validation_features: &'a [vk::ValidationFeatureEnableEXT],
    ) -> Self {
        self.inner.enabled_validation_feature_count = p_enabled_validation_features.len() as u32;
        self.inner.p_enabled_validation_features = p_enabled_validation_features.as_ptr();
        self
    }
    pub fn p_disabled_validation_features(
        mut self,
        p_disabled_validation_features: &'a [vk::ValidationFeatureDisableEXT],
    ) -> Self {
        self.inner.disabled_validation_feature_count = p_disabled_validation_features.len() as u32;
        self.inner.p_disabled_validation_features = p_disabled_validation_features.as_ptr();
        self
    }
}
impl<'a> Deref for ValidationFeaturesEXTBuilder<'a> {
    type Target = vk::ValidationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineRasterizationStateRasterizationOrderAMD {
    type Type = PipelineRasterizationStateRasterizationOrderAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationStateRasterizationOrderAMDBuilder {
    inner: vk::PipelineRasterizationStateRasterizationOrderAMD,
}
impl PipelineRasterizationStateRasterizationOrderAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn rasterization_order(mut self, rasterization_order: vk::RasterizationOrderAMD) -> Self {
        self.inner.rasterization_order = rasterization_order;
        self
    }
}
impl Deref for PipelineRasterizationStateRasterizationOrderAMDBuilder {
    type Target = vk::PipelineRasterizationStateRasterizationOrderAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugMarkerObjectNameInfoEXT {
    type Type = DebugMarkerObjectNameInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerObjectNameInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerObjectNameInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugMarkerObjectNameInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::DebugReportObjectTypeEXT) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object(mut self, object: u64) -> Self {
        self.inner.object = object;
        self
    }
    pub fn p_object_name(mut self, p_object_name: &'a CStr) -> Self {
        self.inner.p_object_name = p_object_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugMarkerObjectNameInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugMarkerObjectTagInfoEXT {
    type Type = DebugMarkerObjectTagInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerObjectTagInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerObjectTagInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugMarkerObjectTagInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::DebugReportObjectTypeEXT) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object(mut self, object: u64) -> Self {
        self.inner.object = object;
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.inner.tag_name = tag_name;
        self
    }
    pub fn p_tag<T>(mut self, p_tag: &'a [T]) -> Self {
        self.inner.tag_size = mem::size_of_val(p_tag) as usize;
        self.inner.p_tag = p_tag.as_ptr() as *const _;
        self
    }
}
impl<'a> Deref for DebugMarkerObjectTagInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerObjectTagInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugMarkerMarkerInfoEXT {
    type Type = DebugMarkerMarkerInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerMarkerInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerMarkerInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugMarkerMarkerInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_marker_name(mut self, p_marker_name: &'a CStr) -> Self {
        self.inner.p_marker_name = p_marker_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugMarkerMarkerInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerMarkerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DedicatedAllocationImageCreateInfoNV {
    type Type = DedicatedAllocationImageCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationImageCreateInfoNVBuilder {
    inner: vk::DedicatedAllocationImageCreateInfoNV,
}
impl DedicatedAllocationImageCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.inner.dedicated_allocation = if dedicated_allocation { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for DedicatedAllocationImageCreateInfoNVBuilder {
    type Target = vk::DedicatedAllocationImageCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DedicatedAllocationBufferCreateInfoNV {
    type Type = DedicatedAllocationBufferCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationBufferCreateInfoNVBuilder {
    inner: vk::DedicatedAllocationBufferCreateInfoNV,
}
impl DedicatedAllocationBufferCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.inner.dedicated_allocation = if dedicated_allocation { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for DedicatedAllocationBufferCreateInfoNVBuilder {
    type Target = vk::DedicatedAllocationBufferCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DedicatedAllocationMemoryAllocateInfoNV {
    type Type = DedicatedAllocationMemoryAllocateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationMemoryAllocateInfoNVBuilder {
    inner: vk::DedicatedAllocationMemoryAllocateInfoNV,
}
impl DedicatedAllocationMemoryAllocateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: Option<vk::Image>) -> Self {
        self.inner.image = image;
        self
    }
    pub fn buffer(mut self, buffer: Option<vk::Buffer>) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl Deref for DedicatedAllocationMemoryAllocateInfoNVBuilder {
    type Target = vk::DedicatedAllocationMemoryAllocateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExternalMemoryImageCreateInfoNV {
    type Type = ExternalMemoryImageCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryImageCreateInfoNVBuilder {
    inner: vk::ExternalMemoryImageCreateInfoNV,
}
impl ExternalMemoryImageCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExternalMemoryImageCreateInfoNVBuilder {
    type Target = vk::ExternalMemoryImageCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExportMemoryAllocateInfoNV {
    type Type = ExportMemoryAllocateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryAllocateInfoNVBuilder {
    inner: vk::ExportMemoryAllocateInfoNV,
}
impl ExportMemoryAllocateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExportMemoryAllocateInfoNVBuilder {
    type Target = vk::ExportMemoryAllocateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportMemoryWin32HandleInfoNV {
    type Type = ImportMemoryWin32HandleInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryWin32HandleInfoNVBuilder {
    inner: vk::ImportMemoryWin32HandleInfoNV,
}
impl ImportMemoryWin32HandleInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
}
impl Deref for ImportMemoryWin32HandleInfoNVBuilder {
    type Target = vk::ImportMemoryWin32HandleInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportMemoryWin32HandleInfoNV {
    type Type = ExportMemoryWin32HandleInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryWin32HandleInfoNVBuilder<'a> {
    inner: vk::ExportMemoryWin32HandleInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ExportMemoryWin32HandleInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
}
impl<'a> Deref for ExportMemoryWin32HandleInfoNVBuilder<'a> {
    type Target = vk::ExportMemoryWin32HandleInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::Win32KeyedMutexAcquireReleaseInfoNV {
    type Type = Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    inner: vk::Win32KeyedMutexAcquireReleaseInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acquire_syncs(
        mut self,
        p_acquire_syncs: &'a [vk::DeviceMemory],
        p_acquire_keys: &'a [u64],
        p_acquire_timeout_milliseconds: &'a [u32],
    ) -> Self {
        self.inner.acquire_count = p_acquire_syncs.len() as u32;
        assert_eq!(self.inner.acquire_count, p_acquire_keys.len() as u32);
        assert_eq!(self.inner.acquire_count, p_acquire_timeout_milliseconds.len() as u32);
        self.inner.p_acquire_syncs = p_acquire_syncs.as_ptr();
        self.inner.p_acquire_keys = p_acquire_keys.as_ptr();
        self.inner.p_acquire_timeout_milliseconds = p_acquire_timeout_milliseconds.as_ptr();
        self
    }
    pub fn p_release_syncs(mut self, p_release_syncs: &'a [vk::DeviceMemory], p_release_keys: &'a [u64]) -> Self {
        self.inner.release_count = p_release_syncs.len() as u32;
        assert_eq!(self.inner.release_count, p_release_keys.len() as u32);
        self.inner.p_release_syncs = p_release_syncs.as_ptr();
        self.inner.p_release_keys = p_release_keys.as_ptr();
        self
    }
}
impl<'a> Deref for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    type Target = vk::Win32KeyedMutexAcquireReleaseInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    type Type = PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder {
    inner: vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV,
}
impl PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_generated_commands(mut self, device_generated_commands: bool) -> Self {
        self.inner.device_generated_commands = if device_generated_commands { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DevicePrivateDataCreateInfoEXT {
    type Type = DevicePrivateDataCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DevicePrivateDataCreateInfoEXTBuilder {
    inner: vk::DevicePrivateDataCreateInfoEXT,
}
impl DevicePrivateDataCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn private_data_slot_request_count(mut self, private_data_slot_request_count: u32) -> Self {
        self.inner.private_data_slot_request_count = private_data_slot_request_count;
        self
    }
}
impl Deref for DevicePrivateDataCreateInfoEXTBuilder {
    type Target = vk::DevicePrivateDataCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PrivateDataSlotCreateInfoEXT {
    type Type = PrivateDataSlotCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PrivateDataSlotCreateInfoEXTBuilder {
    inner: vk::PrivateDataSlotCreateInfoEXT,
}
impl PrivateDataSlotCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PrivateDataSlotCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for PrivateDataSlotCreateInfoEXTBuilder {
    type Target = vk::PrivateDataSlotCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePrivateDataFeaturesEXT {
    type Type = PhysicalDevicePrivateDataFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePrivateDataFeaturesEXTBuilder {
    inner: vk::PhysicalDevicePrivateDataFeaturesEXT,
}
impl PhysicalDevicePrivateDataFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn private_data(mut self, private_data: bool) -> Self {
        self.inner.private_data = if private_data { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevicePrivateDataFeaturesEXTBuilder {
    type Target = vk::PhysicalDevicePrivateDataFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GraphicsShaderGroupCreateInfoNV {
    type Type = GraphicsShaderGroupCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    inner: vk::GraphicsShaderGroupCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_vertex_input_state(
        mut self,
        p_vertex_input_state: Option<&'a vk::PipelineVertexInputStateCreateInfo>,
    ) -> Self {
        self.inner.p_vertex_input_state = p_vertex_input_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_tessellation_state(
        mut self,
        p_tessellation_state: Option<&'a vk::PipelineTessellationStateCreateInfo>,
    ) -> Self {
        self.inner.p_tessellation_state = p_tessellation_state.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    type Target = vk::GraphicsShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GraphicsPipelineShaderGroupsCreateInfoNV {
    type Type = GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    inner: vk::GraphicsPipelineShaderGroupsCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::GraphicsShaderGroupCreateInfoNV]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.as_ptr();
        self
    }
    pub fn p_pipelines(mut self, p_pipelines: &'a [vk::Pipeline]) -> Self {
        self.inner.pipeline_count = p_pipelines.len() as u32;
        self.inner.p_pipelines = p_pipelines.as_ptr();
        self
    }
}
impl<'a> Deref for GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    type Target = vk::GraphicsPipelineShaderGroupsCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutTokenNV {
    type Type = IndirectCommandsLayoutTokenNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IndirectCommandsLayoutTokenNVBuilder<'a> {
    inner: vk::IndirectCommandsLayoutTokenNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> IndirectCommandsLayoutTokenNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn token_type(mut self, token_type: vk::IndirectCommandsTokenTypeNV) -> Self {
        self.inner.token_type = token_type;
        self
    }
    pub fn stream(mut self, stream: u32) -> Self {
        self.inner.stream = stream;
        self
    }
    pub fn offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn vertex_binding_unit(mut self, vertex_binding_unit: u32) -> Self {
        self.inner.vertex_binding_unit = vertex_binding_unit;
        self
    }
    pub fn vertex_dynamic_stride(mut self, vertex_dynamic_stride: bool) -> Self {
        self.inner.vertex_dynamic_stride = if vertex_dynamic_stride { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn pushconstant_pipeline_layout(mut self, pushconstant_pipeline_layout: Option<vk::PipelineLayout>) -> Self {
        self.inner.pushconstant_pipeline_layout = pushconstant_pipeline_layout;
        self
    }
    pub fn pushconstant_shader_stage_flags(mut self, pushconstant_shader_stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.pushconstant_shader_stage_flags = pushconstant_shader_stage_flags;
        self
    }
    pub fn pushconstant_offset(mut self, pushconstant_offset: u32) -> Self {
        self.inner.pushconstant_offset = pushconstant_offset;
        self
    }
    pub fn pushconstant_size(mut self, pushconstant_size: u32) -> Self {
        self.inner.pushconstant_size = pushconstant_size;
        self
    }
    pub fn indirect_state_flags(mut self, indirect_state_flags: vk::IndirectStateFlagsNV) -> Self {
        self.inner.indirect_state_flags = indirect_state_flags;
        self
    }
    pub fn p_index_types(mut self, p_index_types: &'a [vk::IndexType], p_index_type_values: &'a [u32]) -> Self {
        self.inner.index_type_count = p_index_types.len() as u32;
        assert_eq!(self.inner.index_type_count, p_index_type_values.len() as u32);
        self.inner.p_index_types = p_index_types.as_ptr();
        self.inner.p_index_type_values = p_index_type_values.as_ptr();
        self
    }
}
impl<'a> Deref for IndirectCommandsLayoutTokenNVBuilder<'a> {
    type Target = vk::IndirectCommandsLayoutTokenNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutCreateInfoNV {
    type Type = IndirectCommandsLayoutCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    inner: vk::IndirectCommandsLayoutCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::IndirectCommandsLayoutUsageFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn p_tokens(mut self, p_tokens: &'a [vk::IndirectCommandsLayoutTokenNV]) -> Self {
        self.inner.token_count = p_tokens.len() as u32;
        self.inner.p_tokens = p_tokens.as_ptr();
        self
    }
    pub fn p_stream_strides(mut self, p_stream_strides: &'a [u32]) -> Self {
        self.inner.stream_count = p_stream_strides.len() as u32;
        self.inner.p_stream_strides = p_stream_strides.as_ptr();
        self
    }
}
impl<'a> Deref for IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    type Target = vk::IndirectCommandsLayoutCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GeneratedCommandsInfoNV {
    type Type = GeneratedCommandsInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeneratedCommandsInfoNVBuilder<'a> {
    inner: vk::GeneratedCommandsInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> GeneratedCommandsInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn indirect_commands_layout(mut self, indirect_commands_layout: vk::IndirectCommandsLayoutNV) -> Self {
        self.inner.indirect_commands_layout = Some(indirect_commands_layout);
        self
    }
    pub fn p_streams(mut self, p_streams: &'a [vk::IndirectCommandsStreamNV]) -> Self {
        self.inner.stream_count = p_streams.len() as u32;
        self.inner.p_streams = p_streams.as_ptr();
        self
    }
    pub fn sequences_count(mut self, sequences_count: u32) -> Self {
        self.inner.sequences_count = sequences_count;
        self
    }
    pub fn preprocess_buffer(mut self, preprocess_buffer: vk::Buffer) -> Self {
        self.inner.preprocess_buffer = Some(preprocess_buffer);
        self
    }
    pub fn preprocess_offset(mut self, preprocess_offset: vk::DeviceSize) -> Self {
        self.inner.preprocess_offset = preprocess_offset;
        self
    }
    pub fn preprocess_size(mut self, preprocess_size: vk::DeviceSize) -> Self {
        self.inner.preprocess_size = preprocess_size;
        self
    }
    pub fn sequences_count_buffer(mut self, sequences_count_buffer: Option<vk::Buffer>) -> Self {
        self.inner.sequences_count_buffer = sequences_count_buffer;
        self
    }
    pub fn sequences_count_offset(mut self, sequences_count_offset: vk::DeviceSize) -> Self {
        self.inner.sequences_count_offset = sequences_count_offset;
        self
    }
    pub fn sequences_index_buffer(mut self, sequences_index_buffer: Option<vk::Buffer>) -> Self {
        self.inner.sequences_index_buffer = sequences_index_buffer;
        self
    }
    pub fn sequences_index_offset(mut self, sequences_index_offset: vk::DeviceSize) -> Self {
        self.inner.sequences_index_offset = sequences_index_offset;
        self
    }
}
impl<'a> Deref for GeneratedCommandsInfoNVBuilder<'a> {
    type Target = vk::GeneratedCommandsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::GeneratedCommandsMemoryRequirementsInfoNV {
    type Type = GeneratedCommandsMemoryRequirementsInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeneratedCommandsMemoryRequirementsInfoNVBuilder {
    inner: vk::GeneratedCommandsMemoryRequirementsInfoNV,
}
impl GeneratedCommandsMemoryRequirementsInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn indirect_commands_layout(mut self, indirect_commands_layout: vk::IndirectCommandsLayoutNV) -> Self {
        self.inner.indirect_commands_layout = Some(indirect_commands_layout);
        self
    }
    pub fn max_sequences_count(mut self, max_sequences_count: u32) -> Self {
        self.inner.max_sequences_count = max_sequences_count;
        self
    }
}
impl Deref for GeneratedCommandsMemoryRequirementsInfoNVBuilder {
    type Target = vk::GeneratedCommandsMemoryRequirementsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceFeatures2 {
    type Type = PhysicalDeviceFeatures2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFeatures2Builder {
    inner: vk::PhysicalDeviceFeatures2,
}
impl PhysicalDeviceFeatures2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn features(mut self, features: vk::PhysicalDeviceFeatures) -> Self {
        self.inner.features = features;
        self
    }
}
impl Deref for PhysicalDeviceFeatures2Builder {
    type Target = vk::PhysicalDeviceFeatures2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceImageFormatInfo2 {
    type Type = PhysicalDeviceImageFormatInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageFormatInfo2Builder {
    inner: vk::PhysicalDeviceImageFormatInfo2,
}
impl PhysicalDeviceImageFormatInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ty(mut self, ty: vk::ImageType) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for PhysicalDeviceImageFormatInfo2Builder {
    type Target = vk::PhysicalDeviceImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceSparseImageFormatInfo2 {
    type Type = PhysicalDeviceSparseImageFormatInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSparseImageFormatInfo2Builder {
    inner: vk::PhysicalDeviceSparseImageFormatInfo2,
}
impl PhysicalDeviceSparseImageFormatInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ty(mut self, ty: vk::ImageType) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
}
impl Deref for PhysicalDeviceSparseImageFormatInfo2Builder {
    type Target = vk::PhysicalDeviceSparseImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PresentRegionsKHR {
    type Type = PresentRegionsKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentRegionsKHRBuilder<'a> {
    inner: vk::PresentRegionsKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentRegionsKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::PresentRegionKHR]) -> Self {
        self.inner.swapchain_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for PresentRegionsKHRBuilder<'a> {
    type Target = vk::PresentRegionsKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PresentRegionKHR {
    type Type = PresentRegionKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentRegionKHRBuilder<'a> {
    inner: vk::PresentRegionKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentRegionKHRBuilder<'a> {
    pub fn p_rectangles(mut self, p_rectangles: &'a [vk::RectLayerKHR]) -> Self {
        self.inner.rectangle_count = p_rectangles.len() as u32;
        self.inner.p_rectangles = p_rectangles.as_ptr();
        self
    }
}
impl<'a> Deref for PresentRegionKHRBuilder<'a> {
    type Target = vk::PresentRegionKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceVariablePointersFeatures {
    type Type = PhysicalDeviceVariablePointersFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVariablePointersFeaturesBuilder {
    inner: vk::PhysicalDeviceVariablePointersFeatures,
}
impl PhysicalDeviceVariablePointersFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.inner.variable_pointers_storage_buffer = if variable_pointers_storage_buffer {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.inner.variable_pointers = if variable_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceVariablePointersFeaturesBuilder {
    type Target = vk::PhysicalDeviceVariablePointersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceExternalImageFormatInfo {
    type Type = PhysicalDeviceExternalImageFormatInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalImageFormatInfoBuilder {
    inner: vk::PhysicalDeviceExternalImageFormatInfo,
}
impl PhysicalDeviceExternalImageFormatInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for PhysicalDeviceExternalImageFormatInfoBuilder {
    type Target = vk::PhysicalDeviceExternalImageFormatInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceExternalBufferInfo {
    type Type = PhysicalDeviceExternalBufferInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalBufferInfoBuilder {
    inner: vk::PhysicalDeviceExternalBufferInfo,
}
impl PhysicalDeviceExternalBufferInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::BufferCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn usage(mut self, usage: vk::BufferUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for PhysicalDeviceExternalBufferInfoBuilder {
    type Target = vk::PhysicalDeviceExternalBufferInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExternalMemoryImageCreateInfo {
    type Type = ExternalMemoryImageCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryImageCreateInfoBuilder {
    inner: vk::ExternalMemoryImageCreateInfo,
}
impl ExternalMemoryImageCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExternalMemoryImageCreateInfoBuilder {
    type Target = vk::ExternalMemoryImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExternalMemoryBufferCreateInfo {
    type Type = ExternalMemoryBufferCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryBufferCreateInfoBuilder {
    inner: vk::ExternalMemoryBufferCreateInfo,
}
impl ExternalMemoryBufferCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExternalMemoryBufferCreateInfoBuilder {
    type Target = vk::ExternalMemoryBufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExportMemoryAllocateInfo {
    type Type = ExportMemoryAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryAllocateInfoBuilder {
    inner: vk::ExportMemoryAllocateInfo,
}
impl ExportMemoryAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExportMemoryAllocateInfoBuilder {
    type Target = vk::ExportMemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportMemoryWin32HandleInfoKHR {
    type Type = ImportMemoryWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryWin32HandleInfoKHRBuilder {
    inner: vk::ImportMemoryWin32HandleInfoKHR,
}
impl ImportMemoryWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl Deref for ImportMemoryWin32HandleInfoKHRBuilder {
    type Target = vk::ImportMemoryWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportMemoryWin32HandleInfoKHR {
    type Type = ExportMemoryWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportMemoryWin32HandleInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportMemoryWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryGetWin32HandleInfoKHR {
    type Type = MemoryGetWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetWin32HandleInfoKHRBuilder {
    inner: vk::MemoryGetWin32HandleInfoKHR,
}
impl MemoryGetWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for MemoryGetWin32HandleInfoKHRBuilder {
    type Target = vk::MemoryGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportMemoryFdInfoKHR {
    type Type = ImportMemoryFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryFdInfoKHRBuilder {
    inner: vk::ImportMemoryFdInfoKHR,
}
impl ImportMemoryFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl Deref for ImportMemoryFdInfoKHRBuilder {
    type Target = vk::ImportMemoryFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryGetFdInfoKHR {
    type Type = MemoryGetFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetFdInfoKHRBuilder {
    inner: vk::MemoryGetFdInfoKHR,
}
impl MemoryGetFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for MemoryGetFdInfoKHRBuilder {
    type Target = vk::MemoryGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::Win32KeyedMutexAcquireReleaseInfoKHR {
    type Type = Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    inner: vk::Win32KeyedMutexAcquireReleaseInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acquire_syncs(
        mut self,
        p_acquire_syncs: &'a [vk::DeviceMemory],
        p_acquire_keys: &'a [u64],
        p_acquire_timeouts: &'a [u32],
    ) -> Self {
        self.inner.acquire_count = p_acquire_syncs.len() as u32;
        assert_eq!(self.inner.acquire_count, p_acquire_keys.len() as u32);
        assert_eq!(self.inner.acquire_count, p_acquire_timeouts.len() as u32);
        self.inner.p_acquire_syncs = p_acquire_syncs.as_ptr();
        self.inner.p_acquire_keys = p_acquire_keys.as_ptr();
        self.inner.p_acquire_timeouts = p_acquire_timeouts.as_ptr();
        self
    }
    pub fn p_release_syncs(mut self, p_release_syncs: &'a [vk::DeviceMemory], p_release_keys: &'a [u64]) -> Self {
        self.inner.release_count = p_release_syncs.len() as u32;
        assert_eq!(self.inner.release_count, p_release_keys.len() as u32);
        self.inner.p_release_syncs = p_release_syncs.as_ptr();
        self.inner.p_release_keys = p_release_keys.as_ptr();
        self
    }
}
impl<'a> Deref for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    type Target = vk::Win32KeyedMutexAcquireReleaseInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceExternalSemaphoreInfo {
    type Type = PhysicalDeviceExternalSemaphoreInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalSemaphoreInfoBuilder {
    inner: vk::PhysicalDeviceExternalSemaphoreInfo,
}
impl PhysicalDeviceExternalSemaphoreInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for PhysicalDeviceExternalSemaphoreInfoBuilder {
    type Target = vk::PhysicalDeviceExternalSemaphoreInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExportSemaphoreCreateInfo {
    type Type = ExportSemaphoreCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportSemaphoreCreateInfoBuilder {
    inner: vk::ExportSemaphoreCreateInfo,
}
impl ExportSemaphoreCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExportSemaphoreCreateInfoBuilder {
    type Target = vk::ExportSemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportSemaphoreWin32HandleInfoKHR {
    type Type = ImportSemaphoreWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportSemaphoreWin32HandleInfoKHRBuilder {
    inner: vk::ImportSemaphoreWin32HandleInfoKHR,
}
impl ImportSemaphoreWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl Deref for ImportSemaphoreWin32HandleInfoKHRBuilder {
    type Target = vk::ImportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportSemaphoreWin32HandleInfoKHR {
    type Type = ExportSemaphoreWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportSemaphoreWin32HandleInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::D3D12FenceSubmitInfoKHR {
    type Type = D3D12FenceSubmitInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct D3D12FenceSubmitInfoKHRBuilder<'a> {
    inner: vk::D3D12FenceSubmitInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> D3D12FenceSubmitInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_values(mut self, p_wait_semaphore_values: &'a [u64]) -> Self {
        self.inner.wait_semaphore_values_count = p_wait_semaphore_values.len() as u32;
        self.inner.p_wait_semaphore_values = p_wait_semaphore_values.as_ptr();
        self
    }
    pub fn p_signal_semaphore_values(mut self, p_signal_semaphore_values: &'a [u64]) -> Self {
        self.inner.signal_semaphore_values_count = p_signal_semaphore_values.len() as u32;
        self.inner.p_signal_semaphore_values = p_signal_semaphore_values.as_ptr();
        self
    }
}
impl<'a> Deref for D3D12FenceSubmitInfoKHRBuilder<'a> {
    type Target = vk::D3D12FenceSubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreGetWin32HandleInfoKHR {
    type Type = SemaphoreGetWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreGetWin32HandleInfoKHRBuilder {
    inner: vk::SemaphoreGetWin32HandleInfoKHR,
}
impl SemaphoreGetWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for SemaphoreGetWin32HandleInfoKHRBuilder {
    type Target = vk::SemaphoreGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportSemaphoreFdInfoKHR {
    type Type = ImportSemaphoreFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportSemaphoreFdInfoKHRBuilder {
    inner: vk::ImportSemaphoreFdInfoKHR,
}
impl ImportSemaphoreFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl Deref for ImportSemaphoreFdInfoKHRBuilder {
    type Target = vk::ImportSemaphoreFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreGetFdInfoKHR {
    type Type = SemaphoreGetFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreGetFdInfoKHRBuilder {
    inner: vk::SemaphoreGetFdInfoKHR,
}
impl SemaphoreGetFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for SemaphoreGetFdInfoKHRBuilder {
    type Target = vk::SemaphoreGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceExternalFenceInfo {
    type Type = PhysicalDeviceExternalFenceInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalFenceInfoBuilder {
    inner: vk::PhysicalDeviceExternalFenceInfo,
}
impl PhysicalDeviceExternalFenceInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for PhysicalDeviceExternalFenceInfoBuilder {
    type Target = vk::PhysicalDeviceExternalFenceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExportFenceCreateInfo {
    type Type = ExportFenceCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportFenceCreateInfoBuilder {
    inner: vk::ExportFenceCreateInfo,
}
impl ExportFenceCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl Deref for ExportFenceCreateInfoBuilder {
    type Target = vk::ExportFenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportFenceWin32HandleInfoKHR {
    type Type = ImportFenceWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportFenceWin32HandleInfoKHRBuilder {
    inner: vk::ImportFenceWin32HandleInfoKHR,
}
impl ImportFenceWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn flags(mut self, flags: vk::FenceImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl Deref for ImportFenceWin32HandleInfoKHRBuilder {
    type Target = vk::ImportFenceWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportFenceWin32HandleInfoKHR {
    type Type = ExportFenceWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportFenceWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportFenceWin32HandleInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ExportFenceWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportFenceWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportFenceWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::FenceGetWin32HandleInfoKHR {
    type Type = FenceGetWin32HandleInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FenceGetWin32HandleInfoKHRBuilder {
    inner: vk::FenceGetWin32HandleInfoKHR,
}
impl FenceGetWin32HandleInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for FenceGetWin32HandleInfoKHRBuilder {
    type Target = vk::FenceGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportFenceFdInfoKHR {
    type Type = ImportFenceFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportFenceFdInfoKHRBuilder {
    inner: vk::ImportFenceFdInfoKHR,
}
impl ImportFenceFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn flags(mut self, flags: vk::FenceImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl Deref for ImportFenceFdInfoKHRBuilder {
    type Target = vk::ImportFenceFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::FenceGetFdInfoKHR {
    type Type = FenceGetFdInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FenceGetFdInfoKHRBuilder {
    inner: vk::FenceGetFdInfoKHR,
}
impl FenceGetFdInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl Deref for FenceGetFdInfoKHRBuilder {
    type Target = vk::FenceGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceMultiviewFeatures {
    type Type = PhysicalDeviceMultiviewFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMultiviewFeaturesBuilder {
    inner: vk::PhysicalDeviceMultiviewFeatures,
}
impl PhysicalDeviceMultiviewFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.inner.multiview = if multiview { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.inner.multiview_geometry_shader = if multiview_geometry_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.inner.multiview_tessellation_shader = if multiview_tessellation_shader {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceMultiviewFeaturesBuilder {
    type Target = vk::PhysicalDeviceMultiviewFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassMultiviewCreateInfo {
    type Type = RenderPassMultiviewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassMultiviewCreateInfoBuilder<'a> {
    inner: vk::RenderPassMultiviewCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassMultiviewCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_view_masks(mut self, p_view_masks: &'a [u32]) -> Self {
        self.inner.subpass_count = p_view_masks.len() as u32;
        self.inner.p_view_masks = p_view_masks.as_ptr();
        self
    }
    pub fn p_view_offsets(mut self, p_view_offsets: &'a [i32]) -> Self {
        self.inner.dependency_count = p_view_offsets.len() as u32;
        self.inner.p_view_offsets = p_view_offsets.as_ptr();
        self
    }
    pub fn p_correlation_masks(mut self, p_correlation_masks: &'a [u32]) -> Self {
        self.inner.correlation_mask_count = p_correlation_masks.len() as u32;
        self.inner.p_correlation_masks = p_correlation_masks.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassMultiviewCreateInfoBuilder<'a> {
    type Target = vk::RenderPassMultiviewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayPowerInfoEXT {
    type Type = DisplayPowerInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPowerInfoEXTBuilder {
    inner: vk::DisplayPowerInfoEXT,
}
impl DisplayPowerInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn power_state(mut self, power_state: vk::DisplayPowerStateEXT) -> Self {
        self.inner.power_state = power_state;
        self
    }
}
impl Deref for DisplayPowerInfoEXTBuilder {
    type Target = vk::DisplayPowerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceEventInfoEXT {
    type Type = DeviceEventInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceEventInfoEXTBuilder {
    inner: vk::DeviceEventInfoEXT,
}
impl DeviceEventInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_event(mut self, device_event: vk::DeviceEventTypeEXT) -> Self {
        self.inner.device_event = device_event;
        self
    }
}
impl Deref for DeviceEventInfoEXTBuilder {
    type Target = vk::DeviceEventInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayEventInfoEXT {
    type Type = DisplayEventInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayEventInfoEXTBuilder {
    inner: vk::DisplayEventInfoEXT,
}
impl DisplayEventInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn display_event(mut self, display_event: vk::DisplayEventTypeEXT) -> Self {
        self.inner.display_event = display_event;
        self
    }
}
impl Deref for DisplayEventInfoEXTBuilder {
    type Target = vk::DisplayEventInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SwapchainCounterCreateInfoEXT {
    type Type = SwapchainCounterCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainCounterCreateInfoEXTBuilder {
    inner: vk::SwapchainCounterCreateInfoEXT,
}
impl SwapchainCounterCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn surface_counters(mut self, surface_counters: vk::SurfaceCounterFlagsEXT) -> Self {
        self.inner.surface_counters = surface_counters;
        self
    }
}
impl Deref for SwapchainCounterCreateInfoEXTBuilder {
    type Target = vk::SwapchainCounterCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryAllocateFlagsInfo {
    type Type = MemoryAllocateFlagsInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryAllocateFlagsInfoBuilder {
    inner: vk::MemoryAllocateFlagsInfo,
}
impl MemoryAllocateFlagsInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MemoryAllocateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl Deref for MemoryAllocateFlagsInfoBuilder {
    type Target = vk::MemoryAllocateFlagsInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BindBufferMemoryInfo {
    type Type = BindBufferMemoryInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindBufferMemoryInfoBuilder {
    inner: vk::BindBufferMemoryInfo,
}
impl BindBufferMemoryInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
}
impl Deref for BindBufferMemoryInfoBuilder {
    type Target = vk::BindBufferMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindBufferMemoryDeviceGroupInfo {
    type Type = BindBufferMemoryDeviceGroupInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    inner: vk::BindBufferMemoryDeviceGroupInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.as_ptr();
        self
    }
}
impl<'a> Deref for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    type Target = vk::BindBufferMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BindImageMemoryInfo {
    type Type = BindImageMemoryInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImageMemoryInfoBuilder {
    inner: vk::BindImageMemoryInfo,
}
impl BindImageMemoryInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
}
impl Deref for BindImageMemoryInfoBuilder {
    type Target = vk::BindImageMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindImageMemoryDeviceGroupInfo {
    type Type = BindImageMemoryDeviceGroupInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImageMemoryDeviceGroupInfoBuilder<'a> {
    inner: vk::BindImageMemoryDeviceGroupInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindImageMemoryDeviceGroupInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.as_ptr();
        self
    }
    pub fn p_split_instance_bind_regions(mut self, p_split_instance_bind_regions: &'a [vk::Rect2D]) -> Self {
        self.inner.split_instance_bind_region_count = p_split_instance_bind_regions.len() as u32;
        self.inner.p_split_instance_bind_regions = p_split_instance_bind_regions.as_ptr();
        self
    }
}
impl<'a> Deref for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    type Target = vk::BindImageMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceGroupRenderPassBeginInfo {
    type Type = DeviceGroupRenderPassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupRenderPassBeginInfoBuilder<'a> {
    inner: vk::DeviceGroupRenderPassBeginInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceGroupRenderPassBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
    pub fn p_device_render_areas(mut self, p_device_render_areas: &'a [vk::Rect2D]) -> Self {
        self.inner.device_render_area_count = p_device_render_areas.len() as u32;
        self.inner.p_device_render_areas = p_device_render_areas.as_ptr();
        self
    }
}
impl<'a> Deref for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    type Target = vk::DeviceGroupRenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceGroupCommandBufferBeginInfo {
    type Type = DeviceGroupCommandBufferBeginInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupCommandBufferBeginInfoBuilder {
    inner: vk::DeviceGroupCommandBufferBeginInfo,
}
impl DeviceGroupCommandBufferBeginInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl Deref for DeviceGroupCommandBufferBeginInfoBuilder {
    type Target = vk::DeviceGroupCommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceGroupSubmitInfo {
    type Type = DeviceGroupSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupSubmitInfoBuilder<'a> {
    inner: vk::DeviceGroupSubmitInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceGroupSubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_device_indices(mut self, p_wait_semaphore_device_indices: &'a [u32]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphore_device_indices.len() as u32;
        self.inner.p_wait_semaphore_device_indices = p_wait_semaphore_device_indices.as_ptr();
        self
    }
    pub fn p_command_buffer_device_masks(mut self, p_command_buffer_device_masks: &'a [u32]) -> Self {
        self.inner.command_buffer_count = p_command_buffer_device_masks.len() as u32;
        self.inner.p_command_buffer_device_masks = p_command_buffer_device_masks.as_ptr();
        self
    }
    pub fn p_signal_semaphore_device_indices(mut self, p_signal_semaphore_device_indices: &'a [u32]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphore_device_indices.len() as u32;
        self.inner.p_signal_semaphore_device_indices = p_signal_semaphore_device_indices.as_ptr();
        self
    }
}
impl<'a> Deref for DeviceGroupSubmitInfoBuilder<'a> {
    type Target = vk::DeviceGroupSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceGroupBindSparseInfo {
    type Type = DeviceGroupBindSparseInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupBindSparseInfoBuilder {
    inner: vk::DeviceGroupBindSparseInfo,
}
impl DeviceGroupBindSparseInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn resource_device_index(mut self, resource_device_index: u32) -> Self {
        self.inner.resource_device_index = resource_device_index;
        self
    }
    pub fn memory_device_index(mut self, memory_device_index: u32) -> Self {
        self.inner.memory_device_index = memory_device_index;
        self
    }
}
impl Deref for DeviceGroupBindSparseInfoBuilder {
    type Target = vk::DeviceGroupBindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageSwapchainCreateInfoKHR {
    type Type = ImageSwapchainCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageSwapchainCreateInfoKHRBuilder {
    inner: vk::ImageSwapchainCreateInfoKHR,
}
impl ImageSwapchainCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: Option<vk::SwapchainKHR>) -> Self {
        self.inner.swapchain = swapchain;
        self
    }
}
impl Deref for ImageSwapchainCreateInfoKHRBuilder {
    type Target = vk::ImageSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BindImageMemorySwapchainInfoKHR {
    type Type = BindImageMemorySwapchainInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImageMemorySwapchainInfoKHRBuilder {
    inner: vk::BindImageMemorySwapchainInfoKHR,
}
impl BindImageMemorySwapchainInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: vk::SwapchainKHR) -> Self {
        self.inner.swapchain = Some(swapchain);
        self
    }
    pub fn image_index(mut self, image_index: u32) -> Self {
        self.inner.image_index = image_index;
        self
    }
}
impl Deref for BindImageMemorySwapchainInfoKHRBuilder {
    type Target = vk::BindImageMemorySwapchainInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AcquireNextImageInfoKHR {
    type Type = AcquireNextImageInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AcquireNextImageInfoKHRBuilder {
    inner: vk::AcquireNextImageInfoKHR,
}
impl AcquireNextImageInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: vk::SwapchainKHR) -> Self {
        self.inner.swapchain = Some(swapchain);
        self
    }
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.inner.timeout = timeout;
        self
    }
    pub fn semaphore(mut self, semaphore: Option<vk::Semaphore>) -> Self {
        self.inner.semaphore = semaphore;
        self
    }
    pub fn fence(mut self, fence: Option<vk::Fence>) -> Self {
        self.inner.fence = fence;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl Deref for AcquireNextImageInfoKHRBuilder {
    type Target = vk::AcquireNextImageInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceGroupPresentInfoKHR {
    type Type = DeviceGroupPresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupPresentInfoKHRBuilder<'a> {
    inner: vk::DeviceGroupPresentInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceGroupPresentInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_masks(mut self, p_device_masks: &'a [u32]) -> Self {
        self.inner.swapchain_count = p_device_masks.len() as u32;
        self.inner.p_device_masks = p_device_masks.as_ptr();
        self
    }
    pub fn mode(mut self, mode: vk::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl<'a> Deref for DeviceGroupPresentInfoKHRBuilder<'a> {
    type Target = vk::DeviceGroupPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceGroupDeviceCreateInfo {
    type Type = DeviceGroupDeviceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupDeviceCreateInfoBuilder<'a> {
    inner: vk::DeviceGroupDeviceCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DeviceGroupDeviceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_physical_devices(mut self, p_physical_devices: &'a [vk::PhysicalDevice]) -> Self {
        self.inner.physical_device_count = p_physical_devices.len() as u32;
        self.inner.p_physical_devices = p_physical_devices.as_ptr();
        self
    }
}
impl<'a> Deref for DeviceGroupDeviceCreateInfoBuilder<'a> {
    type Target = vk::DeviceGroupDeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceGroupSwapchainCreateInfoKHR {
    type Type = DeviceGroupSwapchainCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupSwapchainCreateInfoKHRBuilder {
    inner: vk::DeviceGroupSwapchainCreateInfoKHR,
}
impl DeviceGroupSwapchainCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn modes(mut self, modes: vk::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.inner.modes = modes;
        self
    }
}
impl Deref for DeviceGroupSwapchainCreateInfoKHRBuilder {
    type Target = vk::DeviceGroupSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorUpdateTemplateCreateInfo {
    type Type = DescriptorUpdateTemplateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    inner: vk::DescriptorUpdateTemplateCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorUpdateTemplateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_descriptor_update_entries(
        mut self,
        p_descriptor_update_entries: &'a [vk::DescriptorUpdateTemplateEntry],
    ) -> Self {
        self.inner.descriptor_update_entry_count = p_descriptor_update_entries.len() as u32;
        self.inner.p_descriptor_update_entries = p_descriptor_update_entries.as_ptr();
        self
    }
    pub fn template_type(mut self, template_type: vk::DescriptorUpdateTemplateType) -> Self {
        self.inner.template_type = template_type;
        self
    }
    pub fn descriptor_set_layout(mut self, descriptor_set_layout: vk::DescriptorSetLayout) -> Self {
        self.inner.descriptor_set_layout = Some(descriptor_set_layout);
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline_layout(mut self, pipeline_layout: vk::PipelineLayout) -> Self {
        self.inner.pipeline_layout = Some(pipeline_layout);
        self
    }
    pub fn set(mut self, set: u32) -> Self {
        self.inner.set = set;
        self
    }
}
impl<'a> Deref for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    type Target = vk::DescriptorUpdateTemplateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::HdrMetadataEXT {
    type Type = HdrMetadataEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct HdrMetadataEXTBuilder {
    inner: vk::HdrMetadataEXT,
}
impl HdrMetadataEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn display_primary_red(mut self, display_primary_red: vk::XYColorEXT) -> Self {
        self.inner.display_primary_red = display_primary_red;
        self
    }
    pub fn display_primary_green(mut self, display_primary_green: vk::XYColorEXT) -> Self {
        self.inner.display_primary_green = display_primary_green;
        self
    }
    pub fn display_primary_blue(mut self, display_primary_blue: vk::XYColorEXT) -> Self {
        self.inner.display_primary_blue = display_primary_blue;
        self
    }
    pub fn white_point(mut self, white_point: vk::XYColorEXT) -> Self {
        self.inner.white_point = white_point;
        self
    }
    pub fn max_luminance(mut self, max_luminance: f32) -> Self {
        self.inner.max_luminance = max_luminance;
        self
    }
    pub fn min_luminance(mut self, min_luminance: f32) -> Self {
        self.inner.min_luminance = min_luminance;
        self
    }
    pub fn max_content_light_level(mut self, max_content_light_level: f32) -> Self {
        self.inner.max_content_light_level = max_content_light_level;
        self
    }
    pub fn max_frame_average_light_level(mut self, max_frame_average_light_level: f32) -> Self {
        self.inner.max_frame_average_light_level = max_frame_average_light_level;
        self
    }
}
impl Deref for HdrMetadataEXTBuilder {
    type Target = vk::HdrMetadataEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SwapchainDisplayNativeHdrCreateInfoAMD {
    type Type = SwapchainDisplayNativeHdrCreateInfoAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMDBuilder {
    inner: vk::SwapchainDisplayNativeHdrCreateInfoAMD,
}
impl SwapchainDisplayNativeHdrCreateInfoAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn local_dimming_enable(mut self, local_dimming_enable: bool) -> Self {
        self.inner.local_dimming_enable = if local_dimming_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for SwapchainDisplayNativeHdrCreateInfoAMDBuilder {
    type Target = vk::SwapchainDisplayNativeHdrCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PresentTimesInfoGOOGLE {
    type Type = PresentTimesInfoGOOGLEBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentTimesInfoGOOGLEBuilder<'a> {
    inner: vk::PresentTimesInfoGOOGLE,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PresentTimesInfoGOOGLEBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_times(mut self, p_times: &'a [vk::PresentTimeGOOGLE]) -> Self {
        self.inner.swapchain_count = p_times.len() as u32;
        self.inner.p_times = p_times.as_ptr();
        self
    }
}
impl<'a> Deref for PresentTimesInfoGOOGLEBuilder<'a> {
    type Target = vk::PresentTimesInfoGOOGLE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::IOSSurfaceCreateInfoMVK {
    type Type = IOSSurfaceCreateInfoMVKBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IOSSurfaceCreateInfoMVKBuilder {
    inner: vk::IOSSurfaceCreateInfoMVK,
}
impl IOSSurfaceCreateInfoMVKBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::IOSSurfaceCreateFlagsMVK) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_view(mut self, p_view: *const c_void) -> Self {
        self.inner.p_view = p_view;
        self
    }
}
impl Deref for IOSSurfaceCreateInfoMVKBuilder {
    type Target = vk::IOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MacOSSurfaceCreateInfoMVK {
    type Type = MacOSSurfaceCreateInfoMVKBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MacOSSurfaceCreateInfoMVKBuilder {
    inner: vk::MacOSSurfaceCreateInfoMVK,
}
impl MacOSSurfaceCreateInfoMVKBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MacOSSurfaceCreateFlagsMVK) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_view(mut self, p_view: *const c_void) -> Self {
        self.inner.p_view = p_view;
        self
    }
}
impl Deref for MacOSSurfaceCreateInfoMVKBuilder {
    type Target = vk::MacOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MetalSurfaceCreateInfoEXT {
    type Type = MetalSurfaceCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MetalSurfaceCreateInfoEXTBuilder<'a> {
    inner: vk::MetalSurfaceCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> MetalSurfaceCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MetalSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_layer(mut self, p_layer: &'a vk::CAMetalLayer) -> Self {
        self.inner.p_layer = p_layer;
        self
    }
}
impl<'a> Deref for MetalSurfaceCreateInfoEXTBuilder<'a> {
    type Target = vk::MetalSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportWScalingStateCreateInfoNV {
    type Type = PipelineViewportWScalingStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportWScalingStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn viewport_w_scaling_enable(mut self, viewport_w_scaling_enable: bool) -> Self {
        self.inner.viewport_w_scaling_enable = if viewport_w_scaling_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.inner.viewport_count = viewport_count;
        self
    }
    pub fn p_viewport_w_scalings(mut self, p_viewport_w_scalings: &'a [vk::ViewportWScalingNV]) -> Self {
        self.inner.viewport_count = p_viewport_w_scalings.len() as u32;
        self.inner.p_viewport_w_scalings = p_viewport_w_scalings.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportWScalingStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportSwizzleStateCreateInfoNV {
    type Type = PipelineViewportSwizzleStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportSwizzleStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineViewportSwizzleStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_viewport_swizzles(mut self, p_viewport_swizzles: &'a [vk::ViewportSwizzleNV]) -> Self {
        self.inner.viewport_count = p_viewport_swizzles.len() as u32;
        self.inner.p_viewport_swizzles = p_viewport_swizzles.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportSwizzleStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineDiscardRectangleStateCreateInfoEXT {
    type Type = PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineDiscardRectangleStateCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDiscardRectangleStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn discard_rectangle_mode(mut self, discard_rectangle_mode: vk::DiscardRectangleModeEXT) -> Self {
        self.inner.discard_rectangle_mode = discard_rectangle_mode;
        self
    }
    pub fn p_discard_rectangles(mut self, p_discard_rectangles: &'a [vk::Rect2D]) -> Self {
        self.inner.discard_rectangle_count = p_discard_rectangles.len() as u32;
        self.inner.p_discard_rectangles = p_discard_rectangles.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineDiscardRectangleStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassInputAttachmentAspectCreateInfo {
    type Type = RenderPassInputAttachmentAspectCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    inner: vk::RenderPassInputAttachmentAspectCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_aspect_references(mut self, p_aspect_references: &'a [vk::InputAttachmentAspectReference]) -> Self {
        self.inner.aspect_reference_count = p_aspect_references.len() as u32;
        self.inner.p_aspect_references = p_aspect_references.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    type Target = vk::RenderPassInputAttachmentAspectCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceSurfaceInfo2KHR {
    type Type = PhysicalDeviceSurfaceInfo2KHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSurfaceInfo2KHRBuilder {
    inner: vk::PhysicalDeviceSurfaceInfo2KHR,
}
impl PhysicalDeviceSurfaceInfo2KHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn surface(mut self, surface: vk::SurfaceKHR) -> Self {
        self.inner.surface = Some(surface);
        self
    }
}
impl Deref for PhysicalDeviceSurfaceInfo2KHRBuilder {
    type Target = vk::PhysicalDeviceSurfaceInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DisplayPlaneInfo2KHR {
    type Type = DisplayPlaneInfo2KHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPlaneInfo2KHRBuilder {
    inner: vk::DisplayPlaneInfo2KHR,
}
impl DisplayPlaneInfo2KHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn mode(mut self, mode: vk::DisplayModeKHR) -> Self {
        self.inner.mode = Some(mode);
        self
    }
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.inner.plane_index = plane_index;
        self
    }
}
impl Deref for DisplayPlaneInfo2KHRBuilder {
    type Target = vk::DisplayPlaneInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevice16BitStorageFeatures {
    type Type = PhysicalDevice16BitStorageFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevice16BitStorageFeaturesBuilder {
    inner: vk::PhysicalDevice16BitStorageFeatures,
}
impl PhysicalDevice16BitStorageFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.inner.storage_buffer16_bit_access = if storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer16_bit_access(mut self, uniform_and_storage_buffer16_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer16_bit_access = if uniform_and_storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.inner.storage_push_constant16 = if storage_push_constant16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.inner.storage_input_output16 = if storage_input_output16 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevice16BitStorageFeaturesBuilder {
    type Target = vk::PhysicalDevice16BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    type Type = PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder {
    inner: vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures,
}
impl PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.inner.shader_subgroup_extended_types = if shader_subgroup_extended_types {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferMemoryRequirementsInfo2 {
    type Type = BufferMemoryRequirementsInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferMemoryRequirementsInfo2Builder {
    inner: vk::BufferMemoryRequirementsInfo2,
}
impl BufferMemoryRequirementsInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
}
impl Deref for BufferMemoryRequirementsInfo2Builder {
    type Target = vk::BufferMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageMemoryRequirementsInfo2 {
    type Type = ImageMemoryRequirementsInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageMemoryRequirementsInfo2Builder {
    inner: vk::ImageMemoryRequirementsInfo2,
}
impl ImageMemoryRequirementsInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
}
impl Deref for ImageMemoryRequirementsInfo2Builder {
    type Target = vk::ImageMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageSparseMemoryRequirementsInfo2 {
    type Type = ImageSparseMemoryRequirementsInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageSparseMemoryRequirementsInfo2Builder {
    inner: vk::ImageSparseMemoryRequirementsInfo2,
}
impl ImageSparseMemoryRequirementsInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
}
impl Deref for ImageSparseMemoryRequirementsInfo2Builder {
    type Target = vk::ImageSparseMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryDedicatedAllocateInfo {
    type Type = MemoryDedicatedAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryDedicatedAllocateInfoBuilder {
    inner: vk::MemoryDedicatedAllocateInfo,
}
impl MemoryDedicatedAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: Option<vk::Image>) -> Self {
        self.inner.image = image;
        self
    }
    pub fn buffer(mut self, buffer: Option<vk::Buffer>) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl Deref for MemoryDedicatedAllocateInfoBuilder {
    type Target = vk::MemoryDedicatedAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageViewUsageCreateInfo {
    type Type = ImageViewUsageCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewUsageCreateInfoBuilder {
    inner: vk::ImageViewUsageCreateInfo,
}
impl ImageViewUsageCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
}
impl Deref for ImageViewUsageCreateInfoBuilder {
    type Target = vk::ImageViewUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineTessellationDomainOriginStateCreateInfo {
    type Type = PipelineTessellationDomainOriginStateCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineTessellationDomainOriginStateCreateInfoBuilder {
    inner: vk::PipelineTessellationDomainOriginStateCreateInfo,
}
impl PipelineTessellationDomainOriginStateCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn domain_origin(mut self, domain_origin: vk::TessellationDomainOrigin) -> Self {
        self.inner.domain_origin = domain_origin;
        self
    }
}
impl Deref for PipelineTessellationDomainOriginStateCreateInfoBuilder {
    type Target = vk::PipelineTessellationDomainOriginStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SamplerYcbcrConversionInfo {
    type Type = SamplerYcbcrConversionInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerYcbcrConversionInfoBuilder {
    inner: vk::SamplerYcbcrConversionInfo,
}
impl SamplerYcbcrConversionInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conversion(mut self, conversion: vk::SamplerYcbcrConversion) -> Self {
        self.inner.conversion = Some(conversion);
        self
    }
}
impl Deref for SamplerYcbcrConversionInfoBuilder {
    type Target = vk::SamplerYcbcrConversionInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SamplerYcbcrConversionCreateInfo {
    type Type = SamplerYcbcrConversionCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerYcbcrConversionCreateInfoBuilder {
    inner: vk::SamplerYcbcrConversionCreateInfo,
}
impl SamplerYcbcrConversionCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ycbcr_model(mut self, ycbcr_model: vk::SamplerYcbcrModelConversion) -> Self {
        self.inner.ycbcr_model = ycbcr_model;
        self
    }
    pub fn ycbcr_range(mut self, ycbcr_range: vk::SamplerYcbcrRange) -> Self {
        self.inner.ycbcr_range = ycbcr_range;
        self
    }
    pub fn components(mut self, components: vk::ComponentMapping) -> Self {
        self.inner.components = components;
        self
    }
    pub fn x_chroma_offset(mut self, x_chroma_offset: vk::ChromaLocation) -> Self {
        self.inner.x_chroma_offset = x_chroma_offset;
        self
    }
    pub fn y_chroma_offset(mut self, y_chroma_offset: vk::ChromaLocation) -> Self {
        self.inner.y_chroma_offset = y_chroma_offset;
        self
    }
    pub fn chroma_filter(mut self, chroma_filter: vk::Filter) -> Self {
        self.inner.chroma_filter = chroma_filter;
        self
    }
    pub fn force_explicit_reconstruction(mut self, force_explicit_reconstruction: bool) -> Self {
        self.inner.force_explicit_reconstruction = if force_explicit_reconstruction {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for SamplerYcbcrConversionCreateInfoBuilder {
    type Target = vk::SamplerYcbcrConversionCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BindImagePlaneMemoryInfo {
    type Type = BindImagePlaneMemoryInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImagePlaneMemoryInfoBuilder {
    inner: vk::BindImagePlaneMemoryInfo,
}
impl BindImagePlaneMemoryInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn plane_aspect(mut self, plane_aspect: vk::ImageAspectFlags) -> Self {
        self.inner.plane_aspect = plane_aspect;
        self
    }
}
impl Deref for BindImagePlaneMemoryInfoBuilder {
    type Target = vk::BindImagePlaneMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImagePlaneMemoryRequirementsInfo {
    type Type = ImagePlaneMemoryRequirementsInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImagePlaneMemoryRequirementsInfoBuilder {
    inner: vk::ImagePlaneMemoryRequirementsInfo,
}
impl ImagePlaneMemoryRequirementsInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn plane_aspect(mut self, plane_aspect: vk::ImageAspectFlags) -> Self {
        self.inner.plane_aspect = plane_aspect;
        self
    }
}
impl Deref for ImagePlaneMemoryRequirementsInfoBuilder {
    type Target = vk::ImagePlaneMemoryRequirementsInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceSamplerYcbcrConversionFeatures {
    type Type = PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder {
    inner: vk::PhysicalDeviceSamplerYcbcrConversionFeatures,
}
impl PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.inner.sampler_ycbcr_conversion = if sampler_ycbcr_conversion { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder {
    type Target = vk::PhysicalDeviceSamplerYcbcrConversionFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ConditionalRenderingBeginInfoEXT {
    type Type = ConditionalRenderingBeginInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ConditionalRenderingBeginInfoEXTBuilder {
    inner: vk::ConditionalRenderingBeginInfoEXT,
}
impl ConditionalRenderingBeginInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn flags(mut self, flags: vk::ConditionalRenderingFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for ConditionalRenderingBeginInfoEXTBuilder {
    type Target = vk::ConditionalRenderingBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ProtectedSubmitInfo {
    type Type = ProtectedSubmitInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ProtectedSubmitInfoBuilder {
    inner: vk::ProtectedSubmitInfo,
}
impl ProtectedSubmitInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn protected_submit(mut self, protected_submit: bool) -> Self {
        self.inner.protected_submit = if protected_submit { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for ProtectedSubmitInfoBuilder {
    type Target = vk::ProtectedSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceProtectedMemoryFeatures {
    type Type = PhysicalDeviceProtectedMemoryFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceProtectedMemoryFeaturesBuilder {
    inner: vk::PhysicalDeviceProtectedMemoryFeatures,
}
impl PhysicalDeviceProtectedMemoryFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.inner.protected_memory = if protected_memory { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceProtectedMemoryFeaturesBuilder {
    type Target = vk::PhysicalDeviceProtectedMemoryFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceQueueInfo2 {
    type Type = DeviceQueueInfo2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceQueueInfo2Builder {
    inner: vk::DeviceQueueInfo2,
}
impl DeviceQueueInfo2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceQueueCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
    pub fn queue_index(mut self, queue_index: u32) -> Self {
        self.inner.queue_index = queue_index;
        self
    }
}
impl Deref for DeviceQueueInfo2Builder {
    type Target = vk::DeviceQueueInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineCoverageToColorStateCreateInfoNV {
    type Type = PipelineCoverageToColorStateCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageToColorStateCreateInfoNVBuilder {
    inner: vk::PipelineCoverageToColorStateCreateInfoNV,
}
impl PipelineCoverageToColorStateCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageToColorStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_to_color_enable(mut self, coverage_to_color_enable: bool) -> Self {
        self.inner.coverage_to_color_enable = if coverage_to_color_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn coverage_to_color_location(mut self, coverage_to_color_location: u32) -> Self {
        self.inner.coverage_to_color_location = coverage_to_color_location;
        self
    }
}
impl Deref for PipelineCoverageToColorStateCreateInfoNVBuilder {
    type Target = vk::PipelineCoverageToColorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SampleLocationsInfoEXT {
    type Type = SampleLocationsInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SampleLocationsInfoEXTBuilder<'a> {
    inner: vk::SampleLocationsInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SampleLocationsInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_locations_per_pixel(mut self, sample_locations_per_pixel: vk::SampleCountFlags) -> Self {
        self.inner.sample_locations_per_pixel = sample_locations_per_pixel;
        self
    }
    pub fn sample_location_grid_size(mut self, sample_location_grid_size: vk::Extent2D) -> Self {
        self.inner.sample_location_grid_size = sample_location_grid_size;
        self
    }
    pub fn p_sample_locations(mut self, p_sample_locations: &'a [vk::SampleLocationEXT]) -> Self {
        self.inner.sample_locations_count = p_sample_locations.len() as u32;
        self.inner.p_sample_locations = p_sample_locations.as_ptr();
        self
    }
}
impl<'a> Deref for SampleLocationsInfoEXTBuilder<'a> {
    type Target = vk::SampleLocationsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassSampleLocationsBeginInfoEXT {
    type Type = RenderPassSampleLocationsBeginInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    inner: vk::RenderPassSampleLocationsBeginInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachment_initial_sample_locations(
        mut self,
        p_attachment_initial_sample_locations: &'a [vk::AttachmentSampleLocationsEXT],
    ) -> Self {
        self.inner.attachment_initial_sample_locations_count = p_attachment_initial_sample_locations.len() as u32;
        self.inner.p_attachment_initial_sample_locations = p_attachment_initial_sample_locations.as_ptr();
        self
    }
    pub fn p_post_subpass_sample_locations(
        mut self,
        p_post_subpass_sample_locations: &'a [vk::SubpassSampleLocationsEXT],
    ) -> Self {
        self.inner.post_subpass_sample_locations_count = p_post_subpass_sample_locations.len() as u32;
        self.inner.p_post_subpass_sample_locations = p_post_subpass_sample_locations.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    type Target = vk::RenderPassSampleLocationsBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineSampleLocationsStateCreateInfoEXT {
    type Type = PipelineSampleLocationsStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineSampleLocationsStateCreateInfoEXTBuilder {
    inner: vk::PipelineSampleLocationsStateCreateInfoEXT,
}
impl PipelineSampleLocationsStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_locations_enable(mut self, sample_locations_enable: bool) -> Self {
        self.inner.sample_locations_enable = if sample_locations_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sample_locations_info(mut self, sample_locations_info: vk::SampleLocationsInfoEXT) -> Self {
        self.inner.sample_locations_info = sample_locations_info;
        self
    }
}
impl Deref for PipelineSampleLocationsStateCreateInfoEXTBuilder {
    type Target = vk::PipelineSampleLocationsStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SamplerReductionModeCreateInfo {
    type Type = SamplerReductionModeCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerReductionModeCreateInfoBuilder {
    inner: vk::SamplerReductionModeCreateInfo,
}
impl SamplerReductionModeCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn reduction_mode(mut self, reduction_mode: vk::SamplerReductionMode) -> Self {
        self.inner.reduction_mode = reduction_mode;
        self
    }
}
impl Deref for SamplerReductionModeCreateInfoBuilder {
    type Target = vk::SamplerReductionModeCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    type Type = PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT,
}
impl PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn advanced_blend_coherent_operations(mut self, advanced_blend_coherent_operations: bool) -> Self {
        self.inner.advanced_blend_coherent_operations = if advanced_blend_coherent_operations {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineColorBlendAdvancedStateCreateInfoEXT {
    type Type = PipelineColorBlendAdvancedStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXTBuilder {
    inner: vk::PipelineColorBlendAdvancedStateCreateInfoEXT,
}
impl PipelineColorBlendAdvancedStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_premultiplied(mut self, src_premultiplied: bool) -> Self {
        self.inner.src_premultiplied = if src_premultiplied { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn dst_premultiplied(mut self, dst_premultiplied: bool) -> Self {
        self.inner.dst_premultiplied = if dst_premultiplied { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn blend_overlap(mut self, blend_overlap: vk::BlendOverlapEXT) -> Self {
        self.inner.blend_overlap = blend_overlap;
        self
    }
}
impl Deref for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder {
    type Target = vk::PipelineColorBlendAdvancedStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceInlineUniformBlockFeaturesEXT {
    type Type = PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceInlineUniformBlockFeaturesEXT,
}
impl PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
        self.inner.inline_uniform_block = if inline_uniform_block { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(
        mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_inline_uniform_block_update_after_bind =
            if descriptor_binding_inline_uniform_block_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceInlineUniformBlockFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetInlineUniformBlockEXT {
    type Type = WriteDescriptorSetInlineUniformBlockEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    inner: vk::WriteDescriptorSetInlineUniformBlockEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_data<T>(mut self, p_data: &'a [T]) -> Self {
        self.inner.data_size = mem::size_of_val(p_data) as u32;
        self.inner.p_data = p_data.as_ptr() as *const _;
        self
    }
}
impl<'a> Deref for WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    type Target = vk::WriteDescriptorSetInlineUniformBlockEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DescriptorPoolInlineUniformBlockCreateInfoEXT {
    type Type = DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder {
    inner: vk::DescriptorPoolInlineUniformBlockCreateInfoEXT,
}
impl DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_inline_uniform_block_bindings(mut self, max_inline_uniform_block_bindings: u32) -> Self {
        self.inner.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings;
        self
    }
}
impl Deref for DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder {
    type Target = vk::DescriptorPoolInlineUniformBlockCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineCoverageModulationStateCreateInfoNV {
    type Type = PipelineCoverageModulationStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineCoverageModulationStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageModulationStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_modulation_mode(mut self, coverage_modulation_mode: vk::CoverageModulationModeNV) -> Self {
        self.inner.coverage_modulation_mode = coverage_modulation_mode;
        self
    }
    pub fn coverage_modulation_table_enable(mut self, coverage_modulation_table_enable: bool) -> Self {
        self.inner.coverage_modulation_table_enable = if coverage_modulation_table_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn coverage_modulation_table_count(mut self, coverage_modulation_table_count: u32) -> Self {
        self.inner.coverage_modulation_table_count = coverage_modulation_table_count;
        self
    }
    pub fn p_coverage_modulation_table(mut self, p_coverage_modulation_table: &'a [f32]) -> Self {
        self.inner.coverage_modulation_table_count = p_coverage_modulation_table.len() as u32;
        self.inner.p_coverage_modulation_table = p_coverage_modulation_table.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineCoverageModulationStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageFormatListCreateInfo {
    type Type = ImageFormatListCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageFormatListCreateInfoBuilder<'a> {
    inner: vk::ImageFormatListCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageFormatListCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_view_formats(mut self, p_view_formats: &'a [vk::Format]) -> Self {
        self.inner.view_format_count = p_view_formats.len() as u32;
        self.inner.p_view_formats = p_view_formats.as_ptr();
        self
    }
}
impl<'a> Deref for ImageFormatListCreateInfoBuilder<'a> {
    type Target = vk::ImageFormatListCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ValidationCacheCreateInfoEXT {
    type Type = ValidationCacheCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationCacheCreateInfoEXTBuilder<'a> {
    inner: vk::ValidationCacheCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ValidationCacheCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ValidationCacheCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_initial_data<T>(mut self, p_initial_data: &'a [T]) -> Self {
        self.inner.initial_data_size = mem::size_of_val(p_initial_data) as usize;
        self.inner.p_initial_data = p_initial_data.as_ptr() as *const _;
        self
    }
}
impl<'a> Deref for ValidationCacheCreateInfoEXTBuilder<'a> {
    type Target = vk::ValidationCacheCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ShaderModuleValidationCacheCreateInfoEXT {
    type Type = ShaderModuleValidationCacheCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ShaderModuleValidationCacheCreateInfoEXTBuilder {
    inner: vk::ShaderModuleValidationCacheCreateInfoEXT,
}
impl ShaderModuleValidationCacheCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn validation_cache(mut self, validation_cache: vk::ValidationCacheEXT) -> Self {
        self.inner.validation_cache = Some(validation_cache);
        self
    }
}
impl Deref for ShaderModuleValidationCacheCreateInfoEXTBuilder {
    type Target = vk::ShaderModuleValidationCacheCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderDrawParametersFeatures {
    type Type = PhysicalDeviceShaderDrawParametersFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderDrawParametersFeaturesBuilder {
    inner: vk::PhysicalDeviceShaderDrawParametersFeatures,
}
impl PhysicalDeviceShaderDrawParametersFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.inner.shader_draw_parameters = if shader_draw_parameters { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderDrawParametersFeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderDrawParametersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderFloat16Int8Features {
    type Type = PhysicalDeviceShaderFloat16Int8FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderFloat16Int8FeaturesBuilder {
    inner: vk::PhysicalDeviceShaderFloat16Int8Features,
}
impl PhysicalDeviceShaderFloat16Int8FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_float16(mut self, shader_float16: bool) -> Self {
        self.inner.shader_float16 = if shader_float16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_int8(mut self, shader_int8: bool) -> Self {
        self.inner.shader_int8 = if shader_int8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderFloat16Int8FeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderFloat16Int8Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceHostQueryResetFeatures {
    type Type = PhysicalDeviceHostQueryResetFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceHostQueryResetFeaturesBuilder {
    inner: vk::PhysicalDeviceHostQueryResetFeatures,
}
impl PhysicalDeviceHostQueryResetFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.inner.host_query_reset = if host_query_reset { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceHostQueryResetFeaturesBuilder {
    type Target = vk::PhysicalDeviceHostQueryResetFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::NativeBufferANDROID {
    type Type = NativeBufferANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct NativeBufferANDROIDBuilder {
    inner: vk::NativeBufferANDROID,
}
impl NativeBufferANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle(mut self, handle: *const c_void) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn stride(mut self, stride: c_int) -> Self {
        self.inner.stride = stride;
        self
    }
    pub fn format(mut self, format: c_int) -> Self {
        self.inner.format = format;
        self
    }
    pub fn usage(mut self, usage: c_int) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn usage2(mut self, usage2: vk::NativeBufferUsage2ANDROID) -> Self {
        self.inner.usage2 = usage2;
        self
    }
}
impl Deref for NativeBufferANDROIDBuilder {
    type Target = vk::NativeBufferANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SwapchainImageCreateInfoANDROID {
    type Type = SwapchainImageCreateInfoANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainImageCreateInfoANDROIDBuilder {
    inner: vk::SwapchainImageCreateInfoANDROID,
}
impl SwapchainImageCreateInfoANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn usage(mut self, usage: vk::SwapchainImageUsageFlagsANDROID) -> Self {
        self.inner.usage = usage;
        self
    }
}
impl Deref for SwapchainImageCreateInfoANDROIDBuilder {
    type Target = vk::SwapchainImageCreateInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePresentationPropertiesANDROID {
    type Type = PhysicalDevicePresentationPropertiesANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePresentationPropertiesANDROIDBuilder {
    inner: vk::PhysicalDevicePresentationPropertiesANDROID,
}
impl PhysicalDevicePresentationPropertiesANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shared_image(mut self, shared_image: bool) -> Self {
        self.inner.shared_image = if shared_image { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevicePresentationPropertiesANDROIDBuilder {
    type Target = vk::PhysicalDevicePresentationPropertiesANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceQueueGlobalPriorityCreateInfoEXT {
    type Type = DeviceQueueGlobalPriorityCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXTBuilder {
    inner: vk::DeviceQueueGlobalPriorityCreateInfoEXT,
}
impl DeviceQueueGlobalPriorityCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn global_priority(mut self, global_priority: vk::QueueGlobalPriorityEXT) -> Self {
        self.inner.global_priority = global_priority;
        self
    }
}
impl Deref for DeviceQueueGlobalPriorityCreateInfoEXTBuilder {
    type Target = vk::DeviceQueueGlobalPriorityCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugUtilsObjectNameInfoEXT {
    type Type = DebugUtilsObjectNameInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsObjectNameInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsObjectNameInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugUtilsObjectNameInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::ObjectType) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.inner.object_handle = object_handle;
        self
    }
    pub fn p_object_name(mut self, p_object_name: Option<&'a CStr>) -> Self {
        self.inner.p_object_name = p_object_name.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
}
impl<'a> Deref for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    type Target = vk::DebugUtilsObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugUtilsObjectTagInfoEXT {
    type Type = DebugUtilsObjectTagInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsObjectTagInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsObjectTagInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugUtilsObjectTagInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::ObjectType) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.inner.object_handle = object_handle;
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.inner.tag_name = tag_name;
        self
    }
    pub fn p_tag<T>(mut self, p_tag: &'a [T]) -> Self {
        self.inner.tag_size = mem::size_of_val(p_tag) as usize;
        self.inner.p_tag = p_tag.as_ptr() as *const _;
        self
    }
}
impl<'a> Deref for DebugUtilsObjectTagInfoEXTBuilder<'a> {
    type Target = vk::DebugUtilsObjectTagInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugUtilsLabelEXT {
    type Type = DebugUtilsLabelEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsLabelEXTBuilder<'a> {
    inner: vk::DebugUtilsLabelEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugUtilsLabelEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_label_name(mut self, p_label_name: &'a CStr) -> Self {
        self.inner.p_label_name = p_label_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugUtilsLabelEXTBuilder<'a> {
    type Target = vk::DebugUtilsLabelEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DebugUtilsMessengerCreateInfoEXT {
    type Type = DebugUtilsMessengerCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsMessengerCreateInfoEXTBuilder {
    inner: vk::DebugUtilsMessengerCreateInfoEXT,
}
impl DebugUtilsMessengerCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugUtilsMessengerCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn message_severity(mut self, message_severity: vk::DebugUtilsMessageSeverityFlagsEXT) -> Self {
        self.inner.message_severity = message_severity;
        self
    }
    pub fn message_type(mut self, message_type: vk::DebugUtilsMessageTypeFlagsEXT) -> Self {
        self.inner.message_type = message_type;
        self
    }
    pub fn pfn_user_callback(mut self, pfn_user_callback: vk::FnDebugUtilsMessengerCallbackEXT) -> Self {
        self.inner.pfn_user_callback = Some(pfn_user_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl Deref for DebugUtilsMessengerCreateInfoEXTBuilder {
    type Target = vk::DebugUtilsMessengerCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugUtilsMessengerCallbackDataEXT {
    type Type = DebugUtilsMessengerCallbackDataEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    inner: vk::DebugUtilsMessengerCallbackDataEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugUtilsMessengerCallbackDataFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_message_id_name(mut self, p_message_id_name: Option<&'a CStr>) -> Self {
        self.inner.p_message_id_name = p_message_id_name.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn message_id_number(mut self, message_id_number: i32) -> Self {
        self.inner.message_id_number = message_id_number;
        self
    }
    pub fn p_message(mut self, p_message: &'a CStr) -> Self {
        self.inner.p_message = p_message.as_ptr();
        self
    }
    pub fn p_queue_labels(mut self, p_queue_labels: &'a [vk::DebugUtilsLabelEXT]) -> Self {
        self.inner.queue_label_count = p_queue_labels.len() as u32;
        self.inner.p_queue_labels = p_queue_labels.as_ptr();
        self
    }
    pub fn p_cmd_buf_labels(mut self, p_cmd_buf_labels: &'a [vk::DebugUtilsLabelEXT]) -> Self {
        self.inner.cmd_buf_label_count = p_cmd_buf_labels.len() as u32;
        self.inner.p_cmd_buf_labels = p_cmd_buf_labels.as_ptr();
        self
    }
    pub fn p_objects(mut self, p_objects: &'a [vk::DebugUtilsObjectNameInfoEXT]) -> Self {
        self.inner.object_count = p_objects.len() as u32;
        self.inner.p_objects = p_objects.as_ptr();
        self
    }
}
impl<'a> Deref for DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    type Target = vk::DebugUtilsMessengerCallbackDataEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    type Type = PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT,
}
impl PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_memory_report(mut self, device_memory_report: bool) -> Self {
        self.inner.device_memory_report = if device_memory_report { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDeviceMemoryReportFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceDeviceMemoryReportCreateInfoEXT {
    type Type = DeviceDeviceMemoryReportCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceDeviceMemoryReportCreateInfoEXTBuilder {
    inner: vk::DeviceDeviceMemoryReportCreateInfoEXT,
}
impl DeviceDeviceMemoryReportCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceMemoryReportFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pfn_user_callback(mut self, pfn_user_callback: vk::FnDeviceMemoryReportCallbackEXT) -> Self {
        self.inner.pfn_user_callback = Some(pfn_user_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl Deref for DeviceDeviceMemoryReportCreateInfoEXTBuilder {
    type Target = vk::DeviceDeviceMemoryReportCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportMemoryHostPointerInfoEXT {
    type Type = ImportMemoryHostPointerInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryHostPointerInfoEXTBuilder {
    inner: vk::ImportMemoryHostPointerInfoEXT,
}
impl ImportMemoryHostPointerInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn p_host_pointer(mut self, p_host_pointer: *mut c_void) -> Self {
        self.inner.p_host_pointer = p_host_pointer;
        self
    }
}
impl Deref for ImportMemoryHostPointerInfoEXTBuilder {
    type Target = vk::ImportMemoryHostPointerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CalibratedTimestampInfoEXT {
    type Type = CalibratedTimestampInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CalibratedTimestampInfoEXTBuilder {
    inner: vk::CalibratedTimestampInfoEXT,
}
impl CalibratedTimestampInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn time_domain(mut self, time_domain: vk::TimeDomainEXT) -> Self {
        self.inner.time_domain = time_domain;
        self
    }
}
impl Deref for CalibratedTimestampInfoEXTBuilder {
    type Target = vk::CalibratedTimestampInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineRasterizationConservativeStateCreateInfoEXT {
    type Type = PipelineRasterizationConservativeStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationConservativeStateCreateInfoEXT,
}
impl PipelineRasterizationConservativeStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationConservativeStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn conservative_rasterization_mode(
        mut self,
        conservative_rasterization_mode: vk::ConservativeRasterizationModeEXT,
    ) -> Self {
        self.inner.conservative_rasterization_mode = conservative_rasterization_mode;
        self
    }
    pub fn extra_primitive_overestimation_size(mut self, extra_primitive_overestimation_size: f32) -> Self {
        self.inner.extra_primitive_overestimation_size = extra_primitive_overestimation_size;
        self
    }
}
impl Deref for PipelineRasterizationConservativeStateCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationConservativeStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceDescriptorIndexingFeatures {
    type Type = PhysicalDeviceDescriptorIndexingFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDescriptorIndexingFeaturesBuilder {
    inner: vk::PhysicalDeviceDescriptorIndexingFeatures,
}
impl PhysicalDeviceDescriptorIndexingFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_dynamic_indexing = if shader_input_attachment_array_dynamic_indexing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_dynamic_indexing =
            if shader_uniform_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_dynamic_indexing =
            if shader_storage_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing =
            if shader_uniform_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_sampled_image_array_non_uniform_indexing = if shader_sampled_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_buffer_array_non_uniform_indexing =
            if shader_storage_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_image_array_non_uniform_indexing = if shader_storage_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_non_uniform_indexing =
            if shader_input_attachment_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_non_uniform_indexing =
            if shader_uniform_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_non_uniform_indexing =
            if shader_storage_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_buffer_update_after_bind =
            if descriptor_binding_uniform_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_sampled_image_update_after_bind =
            if descriptor_binding_sampled_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_image_update_after_bind =
            if descriptor_binding_storage_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_buffer_update_after_bind =
            if descriptor_binding_storage_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_texel_buffer_update_after_bind =
            if descriptor_binding_uniform_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_texel_buffer_update_after_bind =
            if descriptor_binding_storage_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.inner.descriptor_binding_update_unused_while_pending = if descriptor_binding_update_unused_while_pending {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_partially_bound(mut self, descriptor_binding_partially_bound: bool) -> Self {
        self.inner.descriptor_binding_partially_bound = if descriptor_binding_partially_bound {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.inner.descriptor_binding_variable_descriptor_count = if descriptor_binding_variable_descriptor_count {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.inner.runtime_descriptor_array = if runtime_descriptor_array { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDescriptorIndexingFeaturesBuilder {
    type Target = vk::PhysicalDeviceDescriptorIndexingFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutBindingFlagsCreateInfo {
    type Type = DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    inner: vk::DescriptorSetLayoutBindingFlagsCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_binding_flags(mut self, p_binding_flags: &'a [vk::DescriptorBindingFlags]) -> Self {
        self.inner.binding_count = p_binding_flags.len() as u32;
        self.inner.p_binding_flags = p_binding_flags.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    type Target = vk::DescriptorSetLayoutBindingFlagsCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorSetVariableDescriptorCountAllocateInfo {
    type Type = DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    inner: vk::DescriptorSetVariableDescriptorCountAllocateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_descriptor_counts(mut self, p_descriptor_counts: &'a [u32]) -> Self {
        self.inner.descriptor_set_count = p_descriptor_counts.len() as u32;
        self.inner.p_descriptor_counts = p_descriptor_counts.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    type Target = vk::DescriptorSetVariableDescriptorCountAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AttachmentDescription2 {
    type Type = AttachmentDescription2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentDescription2Builder {
    inner: vk::AttachmentDescription2,
}
impl AttachmentDescription2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AttachmentDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn load_op(mut self, load_op: vk::AttachmentLoadOp) -> Self {
        self.inner.load_op = load_op;
        self
    }
    pub fn store_op(mut self, store_op: vk::AttachmentStoreOp) -> Self {
        self.inner.store_op = store_op;
        self
    }
    pub fn stencil_load_op(mut self, stencil_load_op: vk::AttachmentLoadOp) -> Self {
        self.inner.stencil_load_op = stencil_load_op;
        self
    }
    pub fn stencil_store_op(mut self, stencil_store_op: vk::AttachmentStoreOp) -> Self {
        self.inner.stencil_store_op = stencil_store_op;
        self
    }
    pub fn initial_layout(mut self, initial_layout: vk::ImageLayout) -> Self {
        self.inner.initial_layout = initial_layout;
        self
    }
    pub fn final_layout(mut self, final_layout: vk::ImageLayout) -> Self {
        self.inner.final_layout = final_layout;
        self
    }
}
impl Deref for AttachmentDescription2Builder {
    type Target = vk::AttachmentDescription2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AttachmentReference2 {
    type Type = AttachmentReference2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentReference2Builder {
    inner: vk::AttachmentReference2,
}
impl AttachmentReference2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn attachment(mut self, attachment: u32) -> Self {
        self.inner.attachment = attachment;
        self
    }
    pub fn layout(mut self, layout: vk::ImageLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn aspect_mask(mut self, aspect_mask: vk::ImageAspectFlags) -> Self {
        self.inner.aspect_mask = aspect_mask;
        self
    }
}
impl Deref for AttachmentReference2Builder {
    type Target = vk::AttachmentReference2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassDescription2 {
    type Type = SubpassDescription2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDescription2Builder<'a> {
    inner: vk::SubpassDescription2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassDescription2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SubpassDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn p_input_attachments(mut self, p_input_attachments: &'a [vk::AttachmentReference2]) -> Self {
        self.inner.input_attachment_count = p_input_attachments.len() as u32;
        self.inner.p_input_attachments = p_input_attachments.as_ptr();
        self
    }
    pub fn p_color_attachments(
        mut self,
        p_color_attachments: &'a [vk::AttachmentReference2],
        p_resolve_attachments: Option<&'a [vk::AttachmentReference2]>,
    ) -> Self {
        self.inner.color_attachment_count = p_color_attachments.len() as u32;
        if let Some(s) = p_resolve_attachments {
            assert_eq!(self.inner.color_attachment_count, s.len() as u32);
        }
        self.inner.p_color_attachments = p_color_attachments.as_ptr();
        self.inner.p_resolve_attachments = p_resolve_attachments.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn p_depth_stencil_attachment(
        mut self,
        p_depth_stencil_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_depth_stencil_attachment = p_depth_stencil_attachment.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_preserve_attachments(mut self, p_preserve_attachments: &'a [u32]) -> Self {
        self.inner.preserve_attachment_count = p_preserve_attachments.len() as u32;
        self.inner.p_preserve_attachments = p_preserve_attachments.as_ptr();
        self
    }
}
impl<'a> Deref for SubpassDescription2Builder<'a> {
    type Target = vk::SubpassDescription2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SubpassDependency2 {
    type Type = SubpassDependency2Builder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDependency2Builder {
    inner: vk::SubpassDependency2,
}
impl SubpassDependency2Builder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subpass(mut self, src_subpass: u32) -> Self {
        self.inner.src_subpass = src_subpass;
        self
    }
    pub fn dst_subpass(mut self, dst_subpass: u32) -> Self {
        self.inner.dst_subpass = dst_subpass;
        self
    }
    pub fn src_stage_mask(mut self, src_stage_mask: vk::PipelineStageFlags) -> Self {
        self.inner.src_stage_mask = src_stage_mask;
        self
    }
    pub fn dst_stage_mask(mut self, dst_stage_mask: vk::PipelineStageFlags) -> Self {
        self.inner.dst_stage_mask = dst_stage_mask;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn dependency_flags(mut self, dependency_flags: vk::DependencyFlags) -> Self {
        self.inner.dependency_flags = dependency_flags;
        self
    }
    pub fn view_offset(mut self, view_offset: i32) -> Self {
        self.inner.view_offset = view_offset;
        self
    }
}
impl Deref for SubpassDependency2Builder {
    type Target = vk::SubpassDependency2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassCreateInfo2 {
    type Type = RenderPassCreateInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassCreateInfo2Builder<'a> {
    inner: vk::RenderPassCreateInfo2,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassCreateInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderPassCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::AttachmentDescription2]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
    pub fn p_subpasses(mut self, p_subpasses: &'a [vk::SubpassDescription2]) -> Self {
        self.inner.subpass_count = p_subpasses.len() as u32;
        self.inner.p_subpasses = p_subpasses.as_ptr();
        self
    }
    pub fn p_dependencies(mut self, p_dependencies: &'a [vk::SubpassDependency2]) -> Self {
        self.inner.dependency_count = p_dependencies.len() as u32;
        self.inner.p_dependencies = p_dependencies.as_ptr();
        self
    }
    pub fn p_correlated_view_masks(mut self, p_correlated_view_masks: &'a [u32]) -> Self {
        self.inner.correlated_view_mask_count = p_correlated_view_masks.len() as u32;
        self.inner.p_correlated_view_masks = p_correlated_view_masks.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassCreateInfo2Builder<'a> {
    type Target = vk::RenderPassCreateInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SubpassBeginInfo {
    type Type = SubpassBeginInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassBeginInfoBuilder {
    inner: vk::SubpassBeginInfo,
}
impl SubpassBeginInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn contents(mut self, contents: vk::SubpassContents) -> Self {
        self.inner.contents = contents;
        self
    }
}
impl Deref for SubpassBeginInfoBuilder {
    type Target = vk::SubpassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SubpassEndInfo {
    type Type = SubpassEndInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassEndInfoBuilder {
    inner: vk::SubpassEndInfo,
}
impl SubpassEndInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
}
impl Deref for SubpassEndInfoBuilder {
    type Target = vk::SubpassEndInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceTimelineSemaphoreFeatures {
    type Type = PhysicalDeviceTimelineSemaphoreFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTimelineSemaphoreFeaturesBuilder {
    inner: vk::PhysicalDeviceTimelineSemaphoreFeatures,
}
impl PhysicalDeviceTimelineSemaphoreFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.inner.timeline_semaphore = if timeline_semaphore { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceTimelineSemaphoreFeaturesBuilder {
    type Target = vk::PhysicalDeviceTimelineSemaphoreFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreTypeCreateInfo {
    type Type = SemaphoreTypeCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreTypeCreateInfoBuilder {
    inner: vk::SemaphoreTypeCreateInfo,
}
impl SemaphoreTypeCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore_type(mut self, semaphore_type: vk::SemaphoreType) -> Self {
        self.inner.semaphore_type = semaphore_type;
        self
    }
    pub fn initial_value(mut self, initial_value: u64) -> Self {
        self.inner.initial_value = initial_value;
        self
    }
}
impl Deref for SemaphoreTypeCreateInfoBuilder {
    type Target = vk::SemaphoreTypeCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::TimelineSemaphoreSubmitInfo {
    type Type = TimelineSemaphoreSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct TimelineSemaphoreSubmitInfoBuilder<'a> {
    inner: vk::TimelineSemaphoreSubmitInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> TimelineSemaphoreSubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_values(mut self, p_wait_semaphore_values: &'a [u64]) -> Self {
        self.inner.wait_semaphore_value_count = p_wait_semaphore_values.len() as u32;
        self.inner.p_wait_semaphore_values = p_wait_semaphore_values.as_ptr();
        self
    }
    pub fn p_signal_semaphore_values(mut self, p_signal_semaphore_values: &'a [u64]) -> Self {
        self.inner.signal_semaphore_value_count = p_signal_semaphore_values.len() as u32;
        self.inner.p_signal_semaphore_values = p_signal_semaphore_values.as_ptr();
        self
    }
}
impl<'a> Deref for TimelineSemaphoreSubmitInfoBuilder<'a> {
    type Target = vk::TimelineSemaphoreSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SemaphoreWaitInfo {
    type Type = SemaphoreWaitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreWaitInfoBuilder<'a> {
    inner: vk::SemaphoreWaitInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SemaphoreWaitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreWaitFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_semaphores(mut self, p_semaphores: &'a [vk::Semaphore], p_values: &'a [u64]) -> Self {
        self.inner.semaphore_count = p_semaphores.len() as u32;
        assert_eq!(self.inner.semaphore_count, p_values.len() as u32);
        self.inner.p_semaphores = p_semaphores.as_ptr();
        self.inner.p_values = p_values.as_ptr();
        self
    }
}
impl<'a> Deref for SemaphoreWaitInfoBuilder<'a> {
    type Target = vk::SemaphoreWaitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SemaphoreSignalInfo {
    type Type = SemaphoreSignalInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreSignalInfoBuilder {
    inner: vk::SemaphoreSignalInfo,
}
impl SemaphoreSignalInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn value(mut self, value: u64) -> Self {
        self.inner.value = value;
        self
    }
}
impl Deref for SemaphoreSignalInfoBuilder {
    type Target = vk::SemaphoreSignalInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineVertexInputDivisorStateCreateInfoEXT {
    type Type = PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineVertexInputDivisorStateCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_vertex_binding_divisors(
        mut self,
        p_vertex_binding_divisors: &'a [vk::VertexInputBindingDivisorDescriptionEXT],
    ) -> Self {
        self.inner.vertex_binding_divisor_count = p_vertex_binding_divisors.len() as u32;
        self.inner.p_vertex_binding_divisors = p_vertex_binding_divisors.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineVertexInputDivisorStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImportAndroidHardwareBufferInfoANDROID {
    type Type = ImportAndroidHardwareBufferInfoANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportAndroidHardwareBufferInfoANDROIDBuilder {
    inner: vk::ImportAndroidHardwareBufferInfoANDROID,
}
impl ImportAndroidHardwareBufferInfoANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: *mut vk::AHardwareBuffer) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl Deref for ImportAndroidHardwareBufferInfoANDROIDBuilder {
    type Target = vk::ImportAndroidHardwareBufferInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryGetAndroidHardwareBufferInfoANDROID {
    type Type = MemoryGetAndroidHardwareBufferInfoANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROIDBuilder {
    inner: vk::MemoryGetAndroidHardwareBufferInfoANDROID,
}
impl MemoryGetAndroidHardwareBufferInfoANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
}
impl Deref for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder {
    type Target = vk::MemoryGetAndroidHardwareBufferInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CommandBufferInheritanceConditionalRenderingInfoEXT {
    type Type = CommandBufferInheritanceConditionalRenderingInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXTBuilder {
    inner: vk::CommandBufferInheritanceConditionalRenderingInfoEXT,
}
impl CommandBufferInheritanceConditionalRenderingInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conditional_rendering_enable(mut self, conditional_rendering_enable: bool) -> Self {
        self.inner.conditional_rendering_enable = if conditional_rendering_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder {
    type Target = vk::CommandBufferInheritanceConditionalRenderingInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ExternalFormatANDROID {
    type Type = ExternalFormatANDROIDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalFormatANDROIDBuilder {
    inner: vk::ExternalFormatANDROID,
}
impl ExternalFormatANDROIDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn external_format(mut self, external_format: u64) -> Self {
        self.inner.external_format = external_format;
        self
    }
}
impl Deref for ExternalFormatANDROIDBuilder {
    type Target = vk::ExternalFormatANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevice8BitStorageFeatures {
    type Type = PhysicalDevice8BitStorageFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevice8BitStorageFeaturesBuilder {
    inner: vk::PhysicalDevice8BitStorageFeatures,
}
impl PhysicalDevice8BitStorageFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.inner.storage_buffer8_bit_access = if storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer8_bit_access(mut self, uniform_and_storage_buffer8_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer8_bit_access = if uniform_and_storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.inner.storage_push_constant8 = if storage_push_constant8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevice8BitStorageFeaturesBuilder {
    type Target = vk::PhysicalDevice8BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceConditionalRenderingFeaturesEXT {
    type Type = PhysicalDeviceConditionalRenderingFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceConditionalRenderingFeaturesEXT,
}
impl PhysicalDeviceConditionalRenderingFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conditional_rendering(mut self, conditional_rendering: bool) -> Self {
        self.inner.conditional_rendering = if conditional_rendering { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn inherited_conditional_rendering(mut self, inherited_conditional_rendering: bool) -> Self {
        self.inner.inherited_conditional_rendering = if inherited_conditional_rendering {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceConditionalRenderingFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceVulkanMemoryModelFeatures {
    type Type = PhysicalDeviceVulkanMemoryModelFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkanMemoryModelFeaturesBuilder {
    inner: vk::PhysicalDeviceVulkanMemoryModelFeatures,
}
impl PhysicalDeviceVulkanMemoryModelFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.inner.vulkan_memory_model = if vulkan_memory_model { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn vulkan_memory_model_device_scope(mut self, vulkan_memory_model_device_scope: bool) -> Self {
        self.inner.vulkan_memory_model_device_scope = if vulkan_memory_model_device_scope {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.inner.vulkan_memory_model_availability_visibility_chains =
            if vulkan_memory_model_availability_visibility_chains {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDeviceVulkanMemoryModelFeaturesBuilder {
    type Target = vk::PhysicalDeviceVulkanMemoryModelFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderAtomicInt64Features {
    type Type = PhysicalDeviceShaderAtomicInt64FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderAtomicInt64FeaturesBuilder {
    inner: vk::PhysicalDeviceShaderAtomicInt64Features,
}
impl PhysicalDeviceShaderAtomicInt64FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.inner.shader_buffer_int64_atomics = if shader_buffer_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.inner.shader_shared_int64_atomics = if shader_shared_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderAtomicInt64FeaturesBuilder {
    type Target = vk::PhysicalDeviceShaderAtomicInt64Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    type Type = PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT,
}
impl PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_buffer_float32_atomics(mut self, shader_buffer_float32_atomics: bool) -> Self {
        self.inner.shader_buffer_float32_atomics = if shader_buffer_float32_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float32_atomic_add(mut self, shader_buffer_float32_atomic_add: bool) -> Self {
        self.inner.shader_buffer_float32_atomic_add = if shader_buffer_float32_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float64_atomics(mut self, shader_buffer_float64_atomics: bool) -> Self {
        self.inner.shader_buffer_float64_atomics = if shader_buffer_float64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_buffer_float64_atomic_add(mut self, shader_buffer_float64_atomic_add: bool) -> Self {
        self.inner.shader_buffer_float64_atomic_add = if shader_buffer_float64_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float32_atomics(mut self, shader_shared_float32_atomics: bool) -> Self {
        self.inner.shader_shared_float32_atomics = if shader_shared_float32_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float32_atomic_add(mut self, shader_shared_float32_atomic_add: bool) -> Self {
        self.inner.shader_shared_float32_atomic_add = if shader_shared_float32_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float64_atomics(mut self, shader_shared_float64_atomics: bool) -> Self {
        self.inner.shader_shared_float64_atomics = if shader_shared_float64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_float64_atomic_add(mut self, shader_shared_float64_atomic_add: bool) -> Self {
        self.inner.shader_shared_float64_atomic_add = if shader_shared_float64_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_image_float32_atomics(mut self, shader_image_float32_atomics: bool) -> Self {
        self.inner.shader_image_float32_atomics = if shader_image_float32_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_image_float32_atomic_add(mut self, shader_image_float32_atomic_add: bool) -> Self {
        self.inner.shader_image_float32_atomic_add = if shader_image_float32_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn sparse_image_float32_atomics(mut self, sparse_image_float32_atomics: bool) -> Self {
        self.inner.sparse_image_float32_atomics = if sparse_image_float32_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn sparse_image_float32_atomic_add(mut self, sparse_image_float32_atomic_add: bool) -> Self {
        self.inner.sparse_image_float32_atomic_add = if sparse_image_float32_atomic_add {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderAtomicFloatFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    type Type = PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT,
}
impl PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_attribute_instance_rate_divisor(mut self, vertex_attribute_instance_rate_divisor: bool) -> Self {
        self.inner.vertex_attribute_instance_rate_divisor = if vertex_attribute_instance_rate_divisor {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vertex_attribute_instance_rate_zero_divisor(
        mut self,
        vertex_attribute_instance_rate_zero_divisor: bool,
    ) -> Self {
        self.inner.vertex_attribute_instance_rate_zero_divisor = if vertex_attribute_instance_rate_zero_divisor {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassDescriptionDepthStencilResolve {
    type Type = SubpassDescriptionDepthStencilResolveBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDescriptionDepthStencilResolveBuilder<'a> {
    inner: vk::SubpassDescriptionDepthStencilResolve,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> SubpassDescriptionDepthStencilResolveBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn depth_resolve_mode(mut self, depth_resolve_mode: vk::ResolveModeFlags) -> Self {
        self.inner.depth_resolve_mode = depth_resolve_mode;
        self
    }
    pub fn stencil_resolve_mode(mut self, stencil_resolve_mode: vk::ResolveModeFlags) -> Self {
        self.inner.stencil_resolve_mode = stencil_resolve_mode;
        self
    }
    pub fn p_depth_stencil_resolve_attachment(
        mut self,
        p_depth_stencil_resolve_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_depth_stencil_resolve_attachment = p_depth_stencil_resolve_attachment.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for SubpassDescriptionDepthStencilResolveBuilder<'a> {
    type Target = vk::SubpassDescriptionDepthStencilResolve;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageViewASTCDecodeModeEXT {
    type Type = ImageViewASTCDecodeModeEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewASTCDecodeModeEXTBuilder {
    inner: vk::ImageViewASTCDecodeModeEXT,
}
impl ImageViewASTCDecodeModeEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn decode_mode(mut self, decode_mode: vk::Format) -> Self {
        self.inner.decode_mode = decode_mode;
        self
    }
}
impl Deref for ImageViewASTCDecodeModeEXTBuilder {
    type Target = vk::ImageViewASTCDecodeModeEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceASTCDecodeFeaturesEXT {
    type Type = PhysicalDeviceASTCDecodeFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceASTCDecodeFeaturesEXT,
}
impl PhysicalDeviceASTCDecodeFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn decode_mode_shared_exponent(mut self, decode_mode_shared_exponent: bool) -> Self {
        self.inner.decode_mode_shared_exponent = if decode_mode_shared_exponent {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceASTCDecodeFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceASTCDecodeFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceTransformFeedbackFeaturesEXT {
    type Type = PhysicalDeviceTransformFeedbackFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceTransformFeedbackFeaturesEXT,
}
impl PhysicalDeviceTransformFeedbackFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform_feedback(mut self, transform_feedback: bool) -> Self {
        self.inner.transform_feedback = if transform_feedback { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn geometry_streams(mut self, geometry_streams: bool) -> Self {
        self.inner.geometry_streams = if geometry_streams { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceTransformFeedbackFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineRasterizationStateStreamCreateInfoEXT {
    type Type = PipelineRasterizationStateStreamCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationStateStreamCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationStateStreamCreateInfoEXT,
}
impl PipelineRasterizationStateStreamCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationStateStreamCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn rasterization_stream(mut self, rasterization_stream: u32) -> Self {
        self.inner.rasterization_stream = rasterization_stream;
        self
    }
}
impl Deref for PipelineRasterizationStateStreamCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationStateStreamCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    type Type = PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder {
    inner: vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV,
}
impl PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn representative_fragment_test(mut self, representative_fragment_test: bool) -> Self {
        self.inner.representative_fragment_test = if representative_fragment_test {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineRepresentativeFragmentTestStateCreateInfoNV {
    type Type = PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder {
    inner: vk::PipelineRepresentativeFragmentTestStateCreateInfoNV,
}
impl PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn representative_fragment_test_enable(mut self, representative_fragment_test_enable: bool) -> Self {
        self.inner.representative_fragment_test_enable = if representative_fragment_test_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder {
    type Target = vk::PipelineRepresentativeFragmentTestStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceExclusiveScissorFeaturesNV {
    type Type = PhysicalDeviceExclusiveScissorFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNVBuilder {
    inner: vk::PhysicalDeviceExclusiveScissorFeaturesNV,
}
impl PhysicalDeviceExclusiveScissorFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn exclusive_scissor(mut self, exclusive_scissor: bool) -> Self {
        self.inner.exclusive_scissor = if exclusive_scissor { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceExclusiveScissorFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceExclusiveScissorFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportExclusiveScissorStateCreateInfoNV {
    type Type = PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportExclusiveScissorStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_exclusive_scissors(mut self, p_exclusive_scissors: &'a [vk::Rect2D]) -> Self {
        self.inner.exclusive_scissor_count = p_exclusive_scissors.len() as u32;
        self.inner.p_exclusive_scissors = p_exclusive_scissors.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportExclusiveScissorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceCornerSampledImageFeaturesNV {
    type Type = PhysicalDeviceCornerSampledImageFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNVBuilder {
    inner: vk::PhysicalDeviceCornerSampledImageFeaturesNV,
}
impl PhysicalDeviceCornerSampledImageFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn corner_sampled_image(mut self, corner_sampled_image: bool) -> Self {
        self.inner.corner_sampled_image = if corner_sampled_image { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceCornerSampledImageFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceCornerSampledImageFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    type Type = PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder {
    inner: vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV,
}
impl PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compute_derivative_group_quads(mut self, compute_derivative_group_quads: bool) -> Self {
        self.inner.compute_derivative_group_quads = if compute_derivative_group_quads {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn compute_derivative_group_linear(mut self, compute_derivative_group_linear: bool) -> Self {
        self.inner.compute_derivative_group_linear = if compute_derivative_group_linear {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    type Type = PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder {
    inner: vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV,
}
impl PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_shader_barycentric(mut self, fragment_shader_barycentric: bool) -> Self {
        self.inner.fragment_shader_barycentric = if fragment_shader_barycentric {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderImageFootprintFeaturesNV {
    type Type = PhysicalDeviceShaderImageFootprintFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNVBuilder {
    inner: vk::PhysicalDeviceShaderImageFootprintFeaturesNV,
}
impl PhysicalDeviceShaderImageFootprintFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_footprint(mut self, image_footprint: bool) -> Self {
        self.inner.image_footprint = if image_footprint { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceShaderImageFootprintFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    type Type = PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder {
    inner: vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV,
}
impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation_image_aliasing(mut self, dedicated_allocation_image_aliasing: bool) -> Self {
        self.inner.dedicated_allocation_image_aliasing = if dedicated_allocation_image_aliasing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ShadingRatePaletteNV {
    type Type = ShadingRatePaletteNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ShadingRatePaletteNVBuilder<'a> {
    inner: vk::ShadingRatePaletteNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ShadingRatePaletteNVBuilder<'a> {
    pub fn p_shading_rate_palette_entries(
        mut self,
        p_shading_rate_palette_entries: &'a [vk::ShadingRatePaletteEntryNV],
    ) -> Self {
        self.inner.shading_rate_palette_entry_count = p_shading_rate_palette_entries.len() as u32;
        self.inner.p_shading_rate_palette_entries = p_shading_rate_palette_entries.as_ptr();
        self
    }
}
impl<'a> Deref for ShadingRatePaletteNVBuilder<'a> {
    type Target = vk::ShadingRatePaletteNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportShadingRateImageStateCreateInfoNV {
    type Type = PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportShadingRateImageStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_image_enable(mut self, shading_rate_image_enable: bool) -> Self {
        self.inner.shading_rate_image_enable = if shading_rate_image_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn p_shading_rate_palettes(mut self, p_shading_rate_palettes: &'a [vk::ShadingRatePaletteNV]) -> Self {
        self.inner.viewport_count = p_shading_rate_palettes.len() as u32;
        self.inner.p_shading_rate_palettes = p_shading_rate_palettes.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportShadingRateImageStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShadingRateImageFeaturesNV {
    type Type = PhysicalDeviceShadingRateImageFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShadingRateImageFeaturesNVBuilder {
    inner: vk::PhysicalDeviceShadingRateImageFeaturesNV,
}
impl PhysicalDeviceShadingRateImageFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_image(mut self, shading_rate_image: bool) -> Self {
        self.inner.shading_rate_image = if shading_rate_image { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shading_rate_coarse_sample_order(mut self, shading_rate_coarse_sample_order: bool) -> Self {
        self.inner.shading_rate_coarse_sample_order = if shading_rate_coarse_sample_order {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShadingRateImageFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceShadingRateImageFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CoarseSampleOrderCustomNV {
    type Type = CoarseSampleOrderCustomNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CoarseSampleOrderCustomNVBuilder<'a> {
    inner: vk::CoarseSampleOrderCustomNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CoarseSampleOrderCustomNVBuilder<'a> {
    pub fn shading_rate(mut self, shading_rate: vk::ShadingRatePaletteEntryNV) -> Self {
        self.inner.shading_rate = shading_rate;
        self
    }
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.inner.sample_count = sample_count;
        self
    }
    pub fn p_sample_locations(mut self, p_sample_locations: &'a [vk::CoarseSampleLocationNV]) -> Self {
        self.inner.sample_location_count = p_sample_locations.len() as u32;
        self.inner.p_sample_locations = p_sample_locations.as_ptr();
        self
    }
}
impl<'a> Deref for CoarseSampleOrderCustomNVBuilder<'a> {
    type Target = vk::CoarseSampleOrderCustomNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    type Type = PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_order_type(mut self, sample_order_type: vk::CoarseSampleOrderTypeNV) -> Self {
        self.inner.sample_order_type = sample_order_type;
        self
    }
    pub fn p_custom_sample_orders(mut self, p_custom_sample_orders: &'a [vk::CoarseSampleOrderCustomNV]) -> Self {
        self.inner.custom_sample_order_count = p_custom_sample_orders.len() as u32;
        self.inner.p_custom_sample_orders = p_custom_sample_orders.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceMeshShaderFeaturesNV {
    type Type = PhysicalDeviceMeshShaderFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMeshShaderFeaturesNVBuilder {
    inner: vk::PhysicalDeviceMeshShaderFeaturesNV,
}
impl PhysicalDeviceMeshShaderFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn task_shader(mut self, task_shader: bool) -> Self {
        self.inner.task_shader = if task_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn mesh_shader(mut self, mesh_shader: bool) -> Self {
        self.inner.mesh_shader = if mesh_shader { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceMeshShaderFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceMeshShaderFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RayTracingShaderGroupCreateInfoNV {
    type Type = RayTracingShaderGroupCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingShaderGroupCreateInfoNVBuilder {
    inner: vk::RayTracingShaderGroupCreateInfoNV,
}
impl RayTracingShaderGroupCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::RayTracingShaderGroupTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn general_shader(mut self, general_shader: u32) -> Self {
        self.inner.general_shader = general_shader;
        self
    }
    pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
        self.inner.closest_hit_shader = closest_hit_shader;
        self
    }
    pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
        self.inner.any_hit_shader = any_hit_shader;
        self
    }
    pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
        self.inner.intersection_shader = intersection_shader;
        self
    }
}
impl Deref for RayTracingShaderGroupCreateInfoNVBuilder {
    type Target = vk::RayTracingShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RayTracingShaderGroupCreateInfoKHR {
    type Type = RayTracingShaderGroupCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingShaderGroupCreateInfoKHRBuilder {
    inner: vk::RayTracingShaderGroupCreateInfoKHR,
}
impl RayTracingShaderGroupCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::RayTracingShaderGroupTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn general_shader(mut self, general_shader: u32) -> Self {
        self.inner.general_shader = general_shader;
        self
    }
    pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
        self.inner.closest_hit_shader = closest_hit_shader;
        self
    }
    pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
        self.inner.any_hit_shader = any_hit_shader;
        self
    }
    pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
        self.inner.intersection_shader = intersection_shader;
        self
    }
    pub fn p_shader_group_capture_replay_handle(mut self, p_shader_group_capture_replay_handle: *const c_void) -> Self {
        self.inner.p_shader_group_capture_replay_handle = p_shader_group_capture_replay_handle;
        self
    }
}
impl Deref for RayTracingShaderGroupCreateInfoKHRBuilder {
    type Target = vk::RayTracingShaderGroupCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RayTracingPipelineCreateInfoNV {
    type Type = RayTracingPipelineCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingPipelineCreateInfoNVBuilder<'a> {
    inner: vk::RayTracingPipelineCreateInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RayTracingPipelineCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::RayTracingShaderGroupCreateInfoNV]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.as_ptr();
        self
    }
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.inner.max_recursion_depth = max_recursion_depth;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for RayTracingPipelineCreateInfoNVBuilder<'a> {
    type Target = vk::RayTracingPipelineCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RayTracingPipelineCreateInfoKHR {
    type Type = RayTracingPipelineCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingPipelineCreateInfoKHRBuilder<'a> {
    inner: vk::RayTracingPipelineCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RayTracingPipelineCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::RayTracingShaderGroupCreateInfoKHR]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.as_ptr();
        self
    }
    pub fn max_pipeline_ray_recursion_depth(mut self, max_pipeline_ray_recursion_depth: u32) -> Self {
        self.inner.max_pipeline_ray_recursion_depth = max_pipeline_ray_recursion_depth;
        self
    }
    pub fn p_library_info(mut self, p_library_info: Option<&'a vk::PipelineLibraryCreateInfoKHR>) -> Self {
        self.inner.p_library_info = p_library_info.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_library_interface(
        mut self,
        p_library_interface: Option<&'a vk::RayTracingPipelineInterfaceCreateInfoKHR>,
    ) -> Self {
        self.inner.p_library_interface = p_library_interface.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_dynamic_state(mut self, p_dynamic_state: Option<&'a vk::PipelineDynamicStateCreateInfo>) -> Self {
        self.inner.p_dynamic_state = p_dynamic_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for RayTracingPipelineCreateInfoKHRBuilder<'a> {
    type Target = vk::RayTracingPipelineCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::GeometryTrianglesNV {
    type Type = GeometryTrianglesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryTrianglesNVBuilder {
    inner: vk::GeometryTrianglesNV,
}
impl GeometryTrianglesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_data(mut self, vertex_data: Option<vk::Buffer>) -> Self {
        self.inner.vertex_data = vertex_data;
        self
    }
    pub fn vertex_offset(mut self, vertex_offset: vk::DeviceSize) -> Self {
        self.inner.vertex_offset = vertex_offset;
        self
    }
    pub fn vertex_count(mut self, vertex_count: u32) -> Self {
        self.inner.vertex_count = vertex_count;
        self
    }
    pub fn vertex_stride(mut self, vertex_stride: vk::DeviceSize) -> Self {
        self.inner.vertex_stride = vertex_stride;
        self
    }
    pub fn vertex_format(mut self, vertex_format: vk::Format) -> Self {
        self.inner.vertex_format = vertex_format;
        self
    }
    pub fn index_data(mut self, index_data: Option<vk::Buffer>) -> Self {
        self.inner.index_data = index_data;
        self
    }
    pub fn index_offset(mut self, index_offset: vk::DeviceSize) -> Self {
        self.inner.index_offset = index_offset;
        self
    }
    pub fn index_count(mut self, index_count: u32) -> Self {
        self.inner.index_count = index_count;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn transform_data(mut self, transform_data: Option<vk::Buffer>) -> Self {
        self.inner.transform_data = transform_data;
        self
    }
    pub fn transform_offset(mut self, transform_offset: vk::DeviceSize) -> Self {
        self.inner.transform_offset = transform_offset;
        self
    }
}
impl Deref for GeometryTrianglesNVBuilder {
    type Target = vk::GeometryTrianglesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::GeometryAABBNV {
    type Type = GeometryAABBNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryAABBNVBuilder {
    inner: vk::GeometryAABBNV,
}
impl GeometryAABBNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn aabb_data(mut self, aabb_data: Option<vk::Buffer>) -> Self {
        self.inner.aabb_data = aabb_data;
        self
    }
    pub fn num_aab_bs(mut self, num_aab_bs: u32) -> Self {
        self.inner.num_aab_bs = num_aab_bs;
        self
    }
    pub fn stride(mut self, stride: u32) -> Self {
        self.inner.stride = stride;
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
}
impl Deref for GeometryAABBNVBuilder {
    type Target = vk::GeometryAABBNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::GeometryNV {
    type Type = GeometryNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryNVBuilder {
    inner: vk::GeometryNV,
}
impl GeometryNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn geometry_type(mut self, geometry_type: vk::GeometryTypeKHR) -> Self {
        self.inner.geometry_type = geometry_type;
        self
    }
    pub fn geometry(mut self, geometry: vk::GeometryDataNV) -> Self {
        self.inner.geometry = geometry;
        self
    }
    pub fn flags(mut self, flags: vk::GeometryFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for GeometryNVBuilder {
    type Target = vk::GeometryNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureInfoNV {
    type Type = AccelerationStructureInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureInfoNVBuilder<'a> {
    inner: vk::AccelerationStructureInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AccelerationStructureInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureTypeNV) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn flags(mut self, flags: vk::BuildAccelerationStructureFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn instance_count(mut self, instance_count: u32) -> Self {
        self.inner.instance_count = instance_count;
        self
    }
    pub fn p_geometries(mut self, p_geometries: &'a [vk::GeometryNV]) -> Self {
        self.inner.geometry_count = p_geometries.len() as u32;
        self.inner.p_geometries = p_geometries.as_ptr();
        self
    }
}
impl<'a> Deref for AccelerationStructureInfoNVBuilder<'a> {
    type Target = vk::AccelerationStructureInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureCreateInfoNV {
    type Type = AccelerationStructureCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureCreateInfoNVBuilder {
    inner: vk::AccelerationStructureCreateInfoNV,
}
impl AccelerationStructureCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compacted_size(mut self, compacted_size: vk::DeviceSize) -> Self {
        self.inner.compacted_size = compacted_size;
        self
    }
    pub fn info(mut self, info: vk::AccelerationStructureInfoNV) -> Self {
        self.inner.info = info;
        self
    }
}
impl Deref for AccelerationStructureCreateInfoNVBuilder {
    type Target = vk::AccelerationStructureCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindAccelerationStructureMemoryInfoNV {
    type Type = BindAccelerationStructureMemoryInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    inner: vk::BindAccelerationStructureMemoryInfoNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureNV) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.as_ptr();
        self
    }
}
impl<'a> Deref for BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    type Target = vk::BindAccelerationStructureMemoryInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetAccelerationStructureKHR {
    type Type = WriteDescriptorSetAccelerationStructureKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    inner: vk::WriteDescriptorSetAccelerationStructureKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acceleration_structures(mut self, p_acceleration_structures: &'a [vk::AccelerationStructureKHR]) -> Self {
        self.inner.acceleration_structure_count = p_acceleration_structures.len() as u32;
        self.inner.p_acceleration_structures = p_acceleration_structures.as_ptr();
        self
    }
}
impl<'a> Deref for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    type Target = vk::WriteDescriptorSetAccelerationStructureKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetAccelerationStructureNV {
    type Type = WriteDescriptorSetAccelerationStructureNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    inner: vk::WriteDescriptorSetAccelerationStructureNV,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acceleration_structures(mut self, p_acceleration_structures: &'a [vk::AccelerationStructureNV]) -> Self {
        self.inner.acceleration_structure_count = p_acceleration_structures.len() as u32;
        self.inner.p_acceleration_structures = p_acceleration_structures.as_ptr();
        self
    }
}
impl<'a> Deref for WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    type Target = vk::WriteDescriptorSetAccelerationStructureNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureMemoryRequirementsInfoNV {
    type Type = AccelerationStructureMemoryRequirementsInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureMemoryRequirementsInfoNVBuilder {
    inner: vk::AccelerationStructureMemoryRequirementsInfoNV,
}
impl AccelerationStructureMemoryRequirementsInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureMemoryRequirementsTypeNV) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureNV) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
}
impl Deref for AccelerationStructureMemoryRequirementsInfoNVBuilder {
    type Target = vk::AccelerationStructureMemoryRequirementsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceAccelerationStructureFeaturesKHR {
    type Type = PhysicalDeviceAccelerationStructureFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceAccelerationStructureFeaturesKHR,
}
impl PhysicalDeviceAccelerationStructureFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: bool) -> Self {
        self.inner.acceleration_structure = if acceleration_structure { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn acceleration_structure_capture_replay(mut self, acceleration_structure_capture_replay: bool) -> Self {
        self.inner.acceleration_structure_capture_replay = if acceleration_structure_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn acceleration_structure_indirect_build(mut self, acceleration_structure_indirect_build: bool) -> Self {
        self.inner.acceleration_structure_indirect_build = if acceleration_structure_indirect_build {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn acceleration_structure_host_commands(mut self, acceleration_structure_host_commands: bool) -> Self {
        self.inner.acceleration_structure_host_commands = if acceleration_structure_host_commands {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_acceleration_structure_update_after_bind(
        mut self,
        descriptor_binding_acceleration_structure_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_acceleration_structure_update_after_bind =
            if descriptor_binding_acceleration_structure_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl Deref for PhysicalDeviceAccelerationStructureFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceAccelerationStructureFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceRayTracingPipelineFeaturesKHR {
    type Type = PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceRayTracingPipelineFeaturesKHR,
}
impl PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ray_tracing_pipeline(mut self, ray_tracing_pipeline: bool) -> Self {
        self.inner.ray_tracing_pipeline = if ray_tracing_pipeline { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay(
        mut self,
        ray_tracing_pipeline_shader_group_handle_capture_replay: bool,
    ) -> Self {
        self.inner.ray_tracing_pipeline_shader_group_handle_capture_replay =
            if ray_tracing_pipeline_shader_group_handle_capture_replay {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(
        mut self,
        ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: bool,
    ) -> Self {
        self.inner.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed =
            if ray_tracing_pipeline_shader_group_handle_capture_replay_mixed {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn ray_tracing_pipeline_trace_rays_indirect(mut self, ray_tracing_pipeline_trace_rays_indirect: bool) -> Self {
        self.inner.ray_tracing_pipeline_trace_rays_indirect = if ray_tracing_pipeline_trace_rays_indirect {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn ray_traversal_primitive_culling(mut self, ray_traversal_primitive_culling: bool) -> Self {
        self.inner.ray_traversal_primitive_culling = if ray_traversal_primitive_culling {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceRayTracingPipelineFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceRayTracingPipelineFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceRayQueryFeaturesKHR {
    type Type = PhysicalDeviceRayQueryFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRayQueryFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceRayQueryFeaturesKHR,
}
impl PhysicalDeviceRayQueryFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ray_query(mut self, ray_query: bool) -> Self {
        self.inner.ray_query = if ray_query { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceRayQueryFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceRayQueryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceImageDrmFormatModifierInfoEXT {
    type Type = PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    inner: vk::PhysicalDeviceImageDrmFormatModifierInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.inner.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
}
impl<'a> Deref for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageDrmFormatModifierListCreateInfoEXT {
    type Type = ImageDrmFormatModifierListCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    inner: vk::ImageDrmFormatModifierListCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_drm_format_modifiers(mut self, p_drm_format_modifiers: &'a [u64]) -> Self {
        self.inner.drm_format_modifier_count = p_drm_format_modifiers.len() as u32;
        self.inner.p_drm_format_modifiers = p_drm_format_modifiers.as_ptr();
        self
    }
}
impl<'a> Deref for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    type Target = vk::ImageDrmFormatModifierListCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageDrmFormatModifierExplicitCreateInfoEXT {
    type Type = ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    inner: vk::ImageDrmFormatModifierExplicitCreateInfoEXT,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.inner.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn p_plane_layouts(mut self, p_plane_layouts: &'a [vk::SubresourceLayout]) -> Self {
        self.inner.drm_format_modifier_plane_count = p_plane_layouts.len() as u32;
        self.inner.p_plane_layouts = p_plane_layouts.as_ptr();
        self
    }
}
impl<'a> Deref for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    type Target = vk::ImageDrmFormatModifierExplicitCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageStencilUsageCreateInfo {
    type Type = ImageStencilUsageCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageStencilUsageCreateInfoBuilder {
    inner: vk::ImageStencilUsageCreateInfo,
}
impl ImageStencilUsageCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_usage(mut self, stencil_usage: vk::ImageUsageFlags) -> Self {
        self.inner.stencil_usage = stencil_usage;
        self
    }
}
impl Deref for ImageStencilUsageCreateInfoBuilder {
    type Target = vk::ImageStencilUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceMemoryOverallocationCreateInfoAMD {
    type Type = DeviceMemoryOverallocationCreateInfoAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceMemoryOverallocationCreateInfoAMDBuilder {
    inner: vk::DeviceMemoryOverallocationCreateInfoAMD,
}
impl DeviceMemoryOverallocationCreateInfoAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn overallocation_behavior(mut self, overallocation_behavior: vk::MemoryOverallocationBehaviorAMD) -> Self {
        self.inner.overallocation_behavior = overallocation_behavior;
        self
    }
}
impl Deref for DeviceMemoryOverallocationCreateInfoAMDBuilder {
    type Target = vk::DeviceMemoryOverallocationCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceFragmentDensityMapFeaturesEXT {
    type Type = PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceFragmentDensityMapFeaturesEXT,
}
impl PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_density_map(mut self, fragment_density_map: bool) -> Self {
        self.inner.fragment_density_map = if fragment_density_map { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn fragment_density_map_dynamic(mut self, fragment_density_map_dynamic: bool) -> Self {
        self.inner.fragment_density_map_dynamic = if fragment_density_map_dynamic {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn fragment_density_map_non_subsampled_images(
        mut self,
        fragment_density_map_non_subsampled_images: bool,
    ) -> Self {
        self.inner.fragment_density_map_non_subsampled_images = if fragment_density_map_non_subsampled_images {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceFragmentDensityMapFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    type Type = PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT,
}
impl PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_density_map_deferred(mut self, fragment_density_map_deferred: bool) -> Self {
        self.inner.fragment_density_map_deferred = if fragment_density_map_deferred {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentDensityMap2FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RenderPassFragmentDensityMapCreateInfoEXT {
    type Type = RenderPassFragmentDensityMapCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassFragmentDensityMapCreateInfoEXTBuilder {
    inner: vk::RenderPassFragmentDensityMapCreateInfoEXT,
}
impl RenderPassFragmentDensityMapCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_density_map_attachment(mut self, fragment_density_map_attachment: vk::AttachmentReference) -> Self {
        self.inner.fragment_density_map_attachment = fragment_density_map_attachment;
        self
    }
}
impl Deref for RenderPassFragmentDensityMapCreateInfoEXTBuilder {
    type Target = vk::RenderPassFragmentDensityMapCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceScalarBlockLayoutFeatures {
    type Type = PhysicalDeviceScalarBlockLayoutFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceScalarBlockLayoutFeaturesBuilder {
    inner: vk::PhysicalDeviceScalarBlockLayoutFeatures,
}
impl PhysicalDeviceScalarBlockLayoutFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.inner.scalar_block_layout = if scalar_block_layout { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceScalarBlockLayoutFeaturesBuilder {
    type Target = vk::PhysicalDeviceScalarBlockLayoutFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SurfaceProtectedCapabilitiesKHR {
    type Type = SurfaceProtectedCapabilitiesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceProtectedCapabilitiesKHRBuilder {
    inner: vk::SurfaceProtectedCapabilitiesKHR,
}
impl SurfaceProtectedCapabilitiesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn supports_protected(mut self, supports_protected: bool) -> Self {
        self.inner.supports_protected = if supports_protected { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for SurfaceProtectedCapabilitiesKHRBuilder {
    type Target = vk::SurfaceProtectedCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceUniformBufferStandardLayoutFeatures {
    type Type = PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder {
    inner: vk::PhysicalDeviceUniformBufferStandardLayoutFeatures,
}
impl PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.inner.uniform_buffer_standard_layout = if uniform_buffer_standard_layout {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder {
    type Target = vk::PhysicalDeviceUniformBufferStandardLayoutFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceDepthClipEnableFeaturesEXT {
    type Type = PhysicalDeviceDepthClipEnableFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceDepthClipEnableFeaturesEXT,
}
impl PhysicalDeviceDepthClipEnableFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.inner.depth_clip_enable = if depth_clip_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceDepthClipEnableFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineRasterizationDepthClipStateCreateInfoEXT {
    type Type = PipelineRasterizationDepthClipStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationDepthClipStateCreateInfoEXT,
}
impl PipelineRasterizationDepthClipStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationDepthClipStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.inner.depth_clip_enable = if depth_clip_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationDepthClipStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceMemoryPriorityFeaturesEXT {
    type Type = PhysicalDeviceMemoryPriorityFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceMemoryPriorityFeaturesEXT,
}
impl PhysicalDeviceMemoryPriorityFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory_priority(mut self, memory_priority: bool) -> Self {
        self.inner.memory_priority = if memory_priority { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceMemoryPriorityFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryPriorityAllocateInfoEXT {
    type Type = MemoryPriorityAllocateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryPriorityAllocateInfoEXTBuilder {
    inner: vk::MemoryPriorityAllocateInfoEXT,
}
impl MemoryPriorityAllocateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn priority(mut self, priority: f32) -> Self {
        self.inner.priority = priority;
        self
    }
}
impl Deref for MemoryPriorityAllocateInfoEXTBuilder {
    type Target = vk::MemoryPriorityAllocateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceBufferDeviceAddressFeatures {
    type Type = PhysicalDeviceBufferDeviceAddressFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesBuilder {
    inner: vk::PhysicalDeviceBufferDeviceAddressFeatures,
}
impl PhysicalDeviceBufferDeviceAddressFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceBufferDeviceAddressFeaturesBuilder {
    type Target = vk::PhysicalDeviceBufferDeviceAddressFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    type Type = PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT,
}
impl PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferDeviceAddressInfo {
    type Type = BufferDeviceAddressInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferDeviceAddressInfoBuilder {
    inner: vk::BufferDeviceAddressInfo,
}
impl BufferDeviceAddressInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
}
impl Deref for BufferDeviceAddressInfoBuilder {
    type Target = vk::BufferDeviceAddressInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferOpaqueCaptureAddressCreateInfo {
    type Type = BufferOpaqueCaptureAddressCreateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferOpaqueCaptureAddressCreateInfoBuilder {
    inner: vk::BufferOpaqueCaptureAddressCreateInfo,
}
impl BufferOpaqueCaptureAddressCreateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.inner.opaque_capture_address = opaque_capture_address;
        self
    }
}
impl Deref for BufferOpaqueCaptureAddressCreateInfoBuilder {
    type Target = vk::BufferOpaqueCaptureAddressCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferDeviceAddressCreateInfoEXT {
    type Type = BufferDeviceAddressCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferDeviceAddressCreateInfoEXTBuilder {
    inner: vk::BufferDeviceAddressCreateInfoEXT,
}
impl BufferDeviceAddressCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_address(mut self, device_address: vk::DeviceAddress) -> Self {
        self.inner.device_address = device_address;
        self
    }
}
impl Deref for BufferDeviceAddressCreateInfoEXTBuilder {
    type Target = vk::BufferDeviceAddressCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceImageViewImageFormatInfoEXT {
    type Type = PhysicalDeviceImageViewImageFormatInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXTBuilder {
    inner: vk::PhysicalDeviceImageViewImageFormatInfoEXT,
}
impl PhysicalDeviceImageViewImageFormatInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view_type(mut self, image_view_type: vk::ImageViewType) -> Self {
        self.inner.image_view_type = image_view_type;
        self
    }
}
impl Deref for PhysicalDeviceImageViewImageFormatInfoEXTBuilder {
    type Target = vk::PhysicalDeviceImageViewImageFormatInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceImagelessFramebufferFeatures {
    type Type = PhysicalDeviceImagelessFramebufferFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImagelessFramebufferFeaturesBuilder {
    inner: vk::PhysicalDeviceImagelessFramebufferFeatures,
}
impl PhysicalDeviceImagelessFramebufferFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.inner.imageless_framebuffer = if imageless_framebuffer { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceImagelessFramebufferFeaturesBuilder {
    type Target = vk::PhysicalDeviceImagelessFramebufferFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FramebufferAttachmentsCreateInfo {
    type Type = FramebufferAttachmentsCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FramebufferAttachmentsCreateInfoBuilder<'a> {
    inner: vk::FramebufferAttachmentsCreateInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FramebufferAttachmentsCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachment_image_infos(
        mut self,
        p_attachment_image_infos: &'a [vk::FramebufferAttachmentImageInfo],
    ) -> Self {
        self.inner.attachment_image_info_count = p_attachment_image_infos.len() as u32;
        self.inner.p_attachment_image_infos = p_attachment_image_infos.as_ptr();
        self
    }
}
impl<'a> Deref for FramebufferAttachmentsCreateInfoBuilder<'a> {
    type Target = vk::FramebufferAttachmentsCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FramebufferAttachmentImageInfo {
    type Type = FramebufferAttachmentImageInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FramebufferAttachmentImageInfoBuilder<'a> {
    inner: vk::FramebufferAttachmentImageInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FramebufferAttachmentImageInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.inner.layer_count = layer_count;
        self
    }
    pub fn p_view_formats(mut self, p_view_formats: &'a [vk::Format]) -> Self {
        self.inner.view_format_count = p_view_formats.len() as u32;
        self.inner.p_view_formats = p_view_formats.as_ptr();
        self
    }
}
impl<'a> Deref for FramebufferAttachmentImageInfoBuilder<'a> {
    type Target = vk::FramebufferAttachmentImageInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassAttachmentBeginInfo {
    type Type = RenderPassAttachmentBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassAttachmentBeginInfoBuilder<'a> {
    inner: vk::RenderPassAttachmentBeginInfo,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> RenderPassAttachmentBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::ImageView]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassAttachmentBeginInfoBuilder<'a> {
    type Target = vk::RenderPassAttachmentBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    type Type = PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT,
}
impl PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
        self.inner.texture_compression_astc_hdr = if texture_compression_astc_hdr {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceCooperativeMatrixFeaturesNV {
    type Type = PhysicalDeviceCooperativeMatrixFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNVBuilder {
    inner: vk::PhysicalDeviceCooperativeMatrixFeaturesNV,
}
impl PhysicalDeviceCooperativeMatrixFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn cooperative_matrix(mut self, cooperative_matrix: bool) -> Self {
        self.inner.cooperative_matrix = if cooperative_matrix { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn cooperative_matrix_robust_buffer_access(mut self, cooperative_matrix_robust_buffer_access: bool) -> Self {
        self.inner.cooperative_matrix_robust_buffer_access = if cooperative_matrix_robust_buffer_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceCooperativeMatrixFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CooperativeMatrixPropertiesNV {
    type Type = CooperativeMatrixPropertiesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CooperativeMatrixPropertiesNVBuilder {
    inner: vk::CooperativeMatrixPropertiesNV,
}
impl CooperativeMatrixPropertiesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn m_size(mut self, m_size: u32) -> Self {
        self.inner.m_size = m_size;
        self
    }
    pub fn n_size(mut self, n_size: u32) -> Self {
        self.inner.n_size = n_size;
        self
    }
    pub fn k_size(mut self, k_size: u32) -> Self {
        self.inner.k_size = k_size;
        self
    }
    pub fn a_type(mut self, a_type: vk::ComponentTypeNV) -> Self {
        self.inner.a_type = a_type;
        self
    }
    pub fn b_type(mut self, b_type: vk::ComponentTypeNV) -> Self {
        self.inner.b_type = b_type;
        self
    }
    pub fn c_type(mut self, c_type: vk::ComponentTypeNV) -> Self {
        self.inner.c_type = c_type;
        self
    }
    pub fn d_type(mut self, d_type: vk::ComponentTypeNV) -> Self {
        self.inner.d_type = d_type;
        self
    }
    pub fn scope(mut self, scope: vk::ScopeNV) -> Self {
        self.inner.scope = scope;
        self
    }
}
impl Deref for CooperativeMatrixPropertiesNVBuilder {
    type Target = vk::CooperativeMatrixPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    type Type = PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT,
}
impl PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ycbcr_image_arrays(mut self, ycbcr_image_arrays: bool) -> Self {
        self.inner.ycbcr_image_arrays = if ycbcr_image_arrays { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageViewHandleInfoNVX {
    type Type = ImageViewHandleInfoNVXBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewHandleInfoNVXBuilder {
    inner: vk::ImageViewHandleInfoNVX,
}
impl ImageViewHandleInfoNVXBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view(mut self, image_view: vk::ImageView) -> Self {
        self.inner.image_view = Some(image_view);
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
    pub fn sampler(mut self, sampler: Option<vk::Sampler>) -> Self {
        self.inner.sampler = sampler;
        self
    }
}
impl Deref for ImageViewHandleInfoNVXBuilder {
    type Target = vk::ImageViewHandleInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineCreationFeedbackCreateInfoEXT {
    type Type = PipelineCreationFeedbackCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCreationFeedbackCreateInfoEXTBuilder {
    inner: vk::PipelineCreationFeedbackCreateInfoEXT,
}
impl PipelineCreationFeedbackCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_pipeline_creation_feedback(
        mut self,
        p_pipeline_creation_feedback: *mut vk::PipelineCreationFeedbackEXT,
    ) -> Self {
        self.inner.p_pipeline_creation_feedback = p_pipeline_creation_feedback;
        self
    }
    pub fn pipeline_stage_creation_feedback_count(mut self, pipeline_stage_creation_feedback_count: u32) -> Self {
        self.inner.pipeline_stage_creation_feedback_count = pipeline_stage_creation_feedback_count;
        self
    }
    pub fn p_pipeline_stage_creation_feedbacks(
        mut self,
        p_pipeline_stage_creation_feedbacks: *mut vk::PipelineCreationFeedbackEXT,
    ) -> Self {
        self.inner.p_pipeline_stage_creation_feedbacks = p_pipeline_stage_creation_feedbacks;
        self
    }
}
impl Deref for PipelineCreationFeedbackCreateInfoEXTBuilder {
    type Target = vk::PipelineCreationFeedbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SurfaceFullScreenExclusiveInfoEXT {
    type Type = SurfaceFullScreenExclusiveInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceFullScreenExclusiveInfoEXTBuilder {
    inner: vk::SurfaceFullScreenExclusiveInfoEXT,
}
impl SurfaceFullScreenExclusiveInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn full_screen_exclusive(mut self, full_screen_exclusive: vk::FullScreenExclusiveEXT) -> Self {
        self.inner.full_screen_exclusive = full_screen_exclusive;
        self
    }
}
impl Deref for SurfaceFullScreenExclusiveInfoEXTBuilder {
    type Target = vk::SurfaceFullScreenExclusiveInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SurfaceFullScreenExclusiveWin32InfoEXT {
    type Type = SurfaceFullScreenExclusiveWin32InfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXTBuilder {
    inner: vk::SurfaceFullScreenExclusiveWin32InfoEXT,
}
impl SurfaceFullScreenExclusiveWin32InfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn hmonitor(mut self, hmonitor: vk::HMONITOR) -> Self {
        self.inner.hmonitor = hmonitor;
        self
    }
}
impl Deref for SurfaceFullScreenExclusiveWin32InfoEXTBuilder {
    type Target = vk::SurfaceFullScreenExclusiveWin32InfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SurfaceCapabilitiesFullScreenExclusiveEXT {
    type Type = SurfaceCapabilitiesFullScreenExclusiveEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXTBuilder {
    inner: vk::SurfaceCapabilitiesFullScreenExclusiveEXT,
}
impl SurfaceCapabilitiesFullScreenExclusiveEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn full_screen_exclusive_supported(mut self, full_screen_exclusive_supported: bool) -> Self {
        self.inner.full_screen_exclusive_supported = if full_screen_exclusive_supported {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder {
    type Target = vk::SurfaceCapabilitiesFullScreenExclusiveEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePerformanceQueryFeaturesKHR {
    type Type = PhysicalDevicePerformanceQueryFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHRBuilder {
    inner: vk::PhysicalDevicePerformanceQueryFeaturesKHR,
}
impl PhysicalDevicePerformanceQueryFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn performance_counter_query_pools(mut self, performance_counter_query_pools: bool) -> Self {
        self.inner.performance_counter_query_pools = if performance_counter_query_pools {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn performance_counter_multiple_query_pools(mut self, performance_counter_multiple_query_pools: bool) -> Self {
        self.inner.performance_counter_multiple_query_pools = if performance_counter_multiple_query_pools {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDevicePerformanceQueryFeaturesKHRBuilder {
    type Target = vk::PhysicalDevicePerformanceQueryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::QueryPoolPerformanceCreateInfoKHR {
    type Type = QueryPoolPerformanceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    inner: vk::QueryPoolPerformanceCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
    pub fn p_counter_indices(mut self, p_counter_indices: &'a [u32]) -> Self {
        self.inner.counter_index_count = p_counter_indices.len() as u32;
        self.inner.p_counter_indices = p_counter_indices.as_ptr();
        self
    }
}
impl<'a> Deref for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    type Target = vk::QueryPoolPerformanceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AcquireProfilingLockInfoKHR {
    type Type = AcquireProfilingLockInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AcquireProfilingLockInfoKHRBuilder {
    inner: vk::AcquireProfilingLockInfoKHR,
}
impl AcquireProfilingLockInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AcquireProfilingLockFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.inner.timeout = timeout;
        self
    }
}
impl Deref for AcquireProfilingLockInfoKHRBuilder {
    type Target = vk::AcquireProfilingLockInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceQuerySubmitInfoKHR {
    type Type = PerformanceQuerySubmitInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceQuerySubmitInfoKHRBuilder {
    inner: vk::PerformanceQuerySubmitInfoKHR,
}
impl PerformanceQuerySubmitInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn counter_pass_index(mut self, counter_pass_index: u32) -> Self {
        self.inner.counter_pass_index = counter_pass_index;
        self
    }
}
impl Deref for PerformanceQuerySubmitInfoKHRBuilder {
    type Target = vk::PerformanceQuerySubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::HeadlessSurfaceCreateInfoEXT {
    type Type = HeadlessSurfaceCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct HeadlessSurfaceCreateInfoEXTBuilder {
    inner: vk::HeadlessSurfaceCreateInfoEXT,
}
impl HeadlessSurfaceCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::HeadlessSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for HeadlessSurfaceCreateInfoEXTBuilder {
    type Target = vk::HeadlessSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceCoverageReductionModeFeaturesNV {
    type Type = PhysicalDeviceCoverageReductionModeFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNVBuilder {
    inner: vk::PhysicalDeviceCoverageReductionModeFeaturesNV,
}
impl PhysicalDeviceCoverageReductionModeFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: bool) -> Self {
        self.inner.coverage_reduction_mode = if coverage_reduction_mode { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceCoverageReductionModeFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineCoverageReductionStateCreateInfoNV {
    type Type = PipelineCoverageReductionStateCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageReductionStateCreateInfoNVBuilder {
    inner: vk::PipelineCoverageReductionStateCreateInfoNV,
}
impl PipelineCoverageReductionStateCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageReductionStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: vk::CoverageReductionModeNV) -> Self {
        self.inner.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
}
impl Deref for PipelineCoverageReductionStateCreateInfoNVBuilder {
    type Target = vk::PipelineCoverageReductionStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    type Type = PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder {
    inner: vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL,
}
impl PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_integer_functions2(mut self, shader_integer_functions2: bool) -> Self {
        self.inner.shader_integer_functions2 = if shader_integer_functions2 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder {
    type Target = vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::InitializePerformanceApiInfoINTEL {
    type Type = InitializePerformanceApiInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct InitializePerformanceApiInfoINTELBuilder {
    inner: vk::InitializePerformanceApiInfoINTEL,
}
impl InitializePerformanceApiInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl Deref for InitializePerformanceApiInfoINTELBuilder {
    type Target = vk::InitializePerformanceApiInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::QueryPoolPerformanceQueryCreateInfoINTEL {
    type Type = QueryPoolPerformanceQueryCreateInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueryPoolPerformanceQueryCreateInfoINTELBuilder {
    inner: vk::QueryPoolPerformanceQueryCreateInfoINTEL,
}
impl QueryPoolPerformanceQueryCreateInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn performance_counters_sampling(
        mut self,
        performance_counters_sampling: vk::QueryPoolSamplingModeINTEL,
    ) -> Self {
        self.inner.performance_counters_sampling = performance_counters_sampling;
        self
    }
}
impl Deref for QueryPoolPerformanceQueryCreateInfoINTELBuilder {
    type Target = vk::QueryPoolPerformanceQueryCreateInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceMarkerInfoINTEL {
    type Type = PerformanceMarkerInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceMarkerInfoINTELBuilder {
    inner: vk::PerformanceMarkerInfoINTEL,
}
impl PerformanceMarkerInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn marker(mut self, marker: u64) -> Self {
        self.inner.marker = marker;
        self
    }
}
impl Deref for PerformanceMarkerInfoINTELBuilder {
    type Target = vk::PerformanceMarkerInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceStreamMarkerInfoINTEL {
    type Type = PerformanceStreamMarkerInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceStreamMarkerInfoINTELBuilder {
    inner: vk::PerformanceStreamMarkerInfoINTEL,
}
impl PerformanceStreamMarkerInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn marker(mut self, marker: u32) -> Self {
        self.inner.marker = marker;
        self
    }
}
impl Deref for PerformanceStreamMarkerInfoINTELBuilder {
    type Target = vk::PerformanceStreamMarkerInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceOverrideInfoINTEL {
    type Type = PerformanceOverrideInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceOverrideInfoINTELBuilder {
    inner: vk::PerformanceOverrideInfoINTEL,
}
impl PerformanceOverrideInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::PerformanceOverrideTypeINTEL) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn enable(mut self, enable: bool) -> Self {
        self.inner.enable = if enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn parameter(mut self, parameter: u64) -> Self {
        self.inner.parameter = parameter;
        self
    }
}
impl Deref for PerformanceOverrideInfoINTELBuilder {
    type Target = vk::PerformanceOverrideInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PerformanceConfigurationAcquireInfoINTEL {
    type Type = PerformanceConfigurationAcquireInfoINTELBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceConfigurationAcquireInfoINTELBuilder {
    inner: vk::PerformanceConfigurationAcquireInfoINTEL,
}
impl PerformanceConfigurationAcquireInfoINTELBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::PerformanceConfigurationTypeINTEL) -> Self {
        self.inner.ty = ty;
        self
    }
}
impl Deref for PerformanceConfigurationAcquireInfoINTELBuilder {
    type Target = vk::PerformanceConfigurationAcquireInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderClockFeaturesKHR {
    type Type = PhysicalDeviceShaderClockFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderClockFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceShaderClockFeaturesKHR,
}
impl PhysicalDeviceShaderClockFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_subgroup_clock(mut self, shader_subgroup_clock: bool) -> Self {
        self.inner.shader_subgroup_clock = if shader_subgroup_clock { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_device_clock(mut self, shader_device_clock: bool) -> Self {
        self.inner.shader_device_clock = if shader_device_clock { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderClockFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceShaderClockFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceIndexTypeUint8FeaturesEXT {
    type Type = PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceIndexTypeUint8FeaturesEXT,
}
impl PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn index_type_uint8(mut self, index_type_uint8: bool) -> Self {
        self.inner.index_type_uint8 = if index_type_uint8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceIndexTypeUint8FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    type Type = PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder {
    inner: vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV,
}
impl PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_sm_builtins(mut self, shader_sm_builtins: bool) -> Self {
        self.inner.shader_sm_builtins = if shader_sm_builtins { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    type Type = PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT,
}
impl PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_shader_sample_interlock(mut self, fragment_shader_sample_interlock: bool) -> Self {
        self.inner.fragment_shader_sample_interlock = if fragment_shader_sample_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn fragment_shader_pixel_interlock(mut self, fragment_shader_pixel_interlock: bool) -> Self {
        self.inner.fragment_shader_pixel_interlock = if fragment_shader_pixel_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn fragment_shader_shading_rate_interlock(mut self, fragment_shader_shading_rate_interlock: bool) -> Self {
        self.inner.fragment_shader_shading_rate_interlock = if fragment_shader_shading_rate_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    type Type = PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder {
    inner: vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
}
impl PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.inner.separate_depth_stencil_layouts = if separate_depth_stencil_layouts {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder {
    type Target = vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AttachmentReferenceStencilLayout {
    type Type = AttachmentReferenceStencilLayoutBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentReferenceStencilLayoutBuilder {
    inner: vk::AttachmentReferenceStencilLayout,
}
impl AttachmentReferenceStencilLayoutBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_layout(mut self, stencil_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_layout = stencil_layout;
        self
    }
}
impl Deref for AttachmentReferenceStencilLayoutBuilder {
    type Target = vk::AttachmentReferenceStencilLayout;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AttachmentDescriptionStencilLayout {
    type Type = AttachmentDescriptionStencilLayoutBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentDescriptionStencilLayoutBuilder {
    inner: vk::AttachmentDescriptionStencilLayout,
}
impl AttachmentDescriptionStencilLayoutBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_initial_layout(mut self, stencil_initial_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_initial_layout = stencil_initial_layout;
        self
    }
    pub fn stencil_final_layout(mut self, stencil_final_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_final_layout = stencil_final_layout;
        self
    }
}
impl Deref for AttachmentDescriptionStencilLayoutBuilder {
    type Target = vk::AttachmentDescriptionStencilLayout;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    type Type = PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder {
    inner: vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR,
}
impl PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_executable_info(mut self, pipeline_executable_info: bool) -> Self {
        self.inner.pipeline_executable_info = if pipeline_executable_info { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder {
    type Target = vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineInfoKHR {
    type Type = PipelineInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineInfoKHRBuilder {
    inner: vk::PipelineInfoKHR,
}
impl PipelineInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
}
impl Deref for PipelineInfoKHRBuilder {
    type Target = vk::PipelineInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineExecutableInfoKHR {
    type Type = PipelineExecutableInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineExecutableInfoKHRBuilder {
    inner: vk::PipelineExecutableInfoKHR,
}
impl PipelineExecutableInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn executable_index(mut self, executable_index: u32) -> Self {
        self.inner.executable_index = executable_index;
        self
    }
}
impl Deref for PipelineExecutableInfoKHRBuilder {
    type Target = vk::PipelineExecutableInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    type Type = PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT,
}
impl PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_demote_to_helper_invocation(mut self, shader_demote_to_helper_invocation: bool) -> Self {
        self.inner.shader_demote_to_helper_invocation = if shader_demote_to_helper_invocation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    type Type = PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT,
}
impl PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn texel_buffer_alignment(mut self, texel_buffer_alignment: bool) -> Self {
        self.inner.texel_buffer_alignment = if texel_buffer_alignment { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    type Type = PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceSubgroupSizeControlFeaturesEXT,
}
impl PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
        self.inner.subgroup_size_control = if subgroup_size_control { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
        self.inner.compute_full_subgroups = if compute_full_subgroups { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceSubgroupSizeControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::MemoryOpaqueCaptureAddressAllocateInfo {
    type Type = MemoryOpaqueCaptureAddressAllocateInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryOpaqueCaptureAddressAllocateInfoBuilder {
    inner: vk::MemoryOpaqueCaptureAddressAllocateInfo,
}
impl MemoryOpaqueCaptureAddressAllocateInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.inner.opaque_capture_address = opaque_capture_address;
        self
    }
}
impl Deref for MemoryOpaqueCaptureAddressAllocateInfoBuilder {
    type Target = vk::MemoryOpaqueCaptureAddressAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceMemoryOpaqueCaptureAddressInfo {
    type Type = DeviceMemoryOpaqueCaptureAddressInfoBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceMemoryOpaqueCaptureAddressInfoBuilder {
    inner: vk::DeviceMemoryOpaqueCaptureAddressInfo,
}
impl DeviceMemoryOpaqueCaptureAddressInfoBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
}
impl Deref for DeviceMemoryOpaqueCaptureAddressInfoBuilder {
    type Target = vk::DeviceMemoryOpaqueCaptureAddressInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceLineRasterizationFeaturesEXT {
    type Type = PhysicalDeviceLineRasterizationFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceLineRasterizationFeaturesEXT,
}
impl PhysicalDeviceLineRasterizationFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn rectangular_lines(mut self, rectangular_lines: bool) -> Self {
        self.inner.rectangular_lines = if rectangular_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn bresenham_lines(mut self, bresenham_lines: bool) -> Self {
        self.inner.bresenham_lines = if bresenham_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn smooth_lines(mut self, smooth_lines: bool) -> Self {
        self.inner.smooth_lines = if smooth_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: bool) -> Self {
        self.inner.stippled_rectangular_lines = if stippled_rectangular_lines {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: bool) -> Self {
        self.inner.stippled_bresenham_lines = if stippled_bresenham_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: bool) -> Self {
        self.inner.stippled_smooth_lines = if stippled_smooth_lines { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceLineRasterizationFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceLineRasterizationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineRasterizationLineStateCreateInfoEXT {
    type Type = PipelineRasterizationLineStateCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationLineStateCreateInfoEXTBuilder {
    inner: vk::PipelineRasterizationLineStateCreateInfoEXT,
}
impl PipelineRasterizationLineStateCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn line_rasterization_mode(mut self, line_rasterization_mode: vk::LineRasterizationModeEXT) -> Self {
        self.inner.line_rasterization_mode = line_rasterization_mode;
        self
    }
    pub fn stippled_line_enable(mut self, stippled_line_enable: bool) -> Self {
        self.inner.stippled_line_enable = if stippled_line_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn line_stipple_factor(mut self, line_stipple_factor: u32) -> Self {
        self.inner.line_stipple_factor = line_stipple_factor;
        self
    }
    pub fn line_stipple_pattern(mut self, line_stipple_pattern: u16) -> Self {
        self.inner.line_stipple_pattern = line_stipple_pattern;
        self
    }
}
impl Deref for PipelineRasterizationLineStateCreateInfoEXTBuilder {
    type Target = vk::PipelineRasterizationLineStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    type Type = PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder {
    inner: vk::PhysicalDevicePipelineCreationCacheControlFeaturesEXT,
}
impl PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_creation_cache_control(mut self, pipeline_creation_cache_control: bool) -> Self {
        self.inner.pipeline_creation_cache_control = if pipeline_creation_cache_control {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder {
    type Target = vk::PhysicalDevicePipelineCreationCacheControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceVulkan11Features {
    type Type = PhysicalDeviceVulkan11FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkan11FeaturesBuilder {
    inner: vk::PhysicalDeviceVulkan11Features,
}
impl PhysicalDeviceVulkan11FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.inner.storage_buffer16_bit_access = if storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer16_bit_access(mut self, uniform_and_storage_buffer16_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer16_bit_access = if uniform_and_storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.inner.storage_push_constant16 = if storage_push_constant16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.inner.storage_input_output16 = if storage_input_output16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.inner.multiview = if multiview { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.inner.multiview_geometry_shader = if multiview_geometry_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.inner.multiview_tessellation_shader = if multiview_tessellation_shader {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.inner.variable_pointers_storage_buffer = if variable_pointers_storage_buffer {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.inner.variable_pointers = if variable_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.inner.protected_memory = if protected_memory { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.inner.sampler_ycbcr_conversion = if sampler_ycbcr_conversion { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.inner.shader_draw_parameters = if shader_draw_parameters { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceVulkan11FeaturesBuilder {
    type Target = vk::PhysicalDeviceVulkan11Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceVulkan12Features {
    type Type = PhysicalDeviceVulkan12FeaturesBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkan12FeaturesBuilder {
    inner: vk::PhysicalDeviceVulkan12Features,
}
impl PhysicalDeviceVulkan12FeaturesBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sampler_mirror_clamp_to_edge(mut self, sampler_mirror_clamp_to_edge: bool) -> Self {
        self.inner.sampler_mirror_clamp_to_edge = if sampler_mirror_clamp_to_edge {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn draw_indirect_count(mut self, draw_indirect_count: bool) -> Self {
        self.inner.draw_indirect_count = if draw_indirect_count { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.inner.storage_buffer8_bit_access = if storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer8_bit_access(mut self, uniform_and_storage_buffer8_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer8_bit_access = if uniform_and_storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.inner.storage_push_constant8 = if storage_push_constant8 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.inner.shader_buffer_int64_atomics = if shader_buffer_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.inner.shader_shared_int64_atomics = if shader_shared_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_float16(mut self, shader_float16: bool) -> Self {
        self.inner.shader_float16 = if shader_float16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_int8(mut self, shader_int8: bool) -> Self {
        self.inner.shader_int8 = if shader_int8 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn descriptor_indexing(mut self, descriptor_indexing: bool) -> Self {
        self.inner.descriptor_indexing = if descriptor_indexing { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_dynamic_indexing = if shader_input_attachment_array_dynamic_indexing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_dynamic_indexing =
            if shader_uniform_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_dynamic_indexing =
            if shader_storage_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing =
            if shader_uniform_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_sampled_image_array_non_uniform_indexing = if shader_sampled_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_buffer_array_non_uniform_indexing =
            if shader_storage_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_image_array_non_uniform_indexing = if shader_storage_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_non_uniform_indexing =
            if shader_input_attachment_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_non_uniform_indexing =
            if shader_uniform_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_non_uniform_indexing =
            if shader_storage_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_buffer_update_after_bind =
            if descriptor_binding_uniform_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_sampled_image_update_after_bind =
            if descriptor_binding_sampled_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_image_update_after_bind =
            if descriptor_binding_storage_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_buffer_update_after_bind =
            if descriptor_binding_storage_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_texel_buffer_update_after_bind =
            if descriptor_binding_uniform_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_texel_buffer_update_after_bind =
            if descriptor_binding_storage_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.inner.descriptor_binding_update_unused_while_pending = if descriptor_binding_update_unused_while_pending {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_partially_bound(mut self, descriptor_binding_partially_bound: bool) -> Self {
        self.inner.descriptor_binding_partially_bound = if descriptor_binding_partially_bound {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.inner.descriptor_binding_variable_descriptor_count = if descriptor_binding_variable_descriptor_count {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.inner.runtime_descriptor_array = if runtime_descriptor_array { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sampler_filter_minmax(mut self, sampler_filter_minmax: bool) -> Self {
        self.inner.sampler_filter_minmax = if sampler_filter_minmax { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.inner.scalar_block_layout = if scalar_block_layout { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.inner.imageless_framebuffer = if imageless_framebuffer { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.inner.uniform_buffer_standard_layout = if uniform_buffer_standard_layout {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.inner.shader_subgroup_extended_types = if shader_subgroup_extended_types {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.inner.separate_depth_stencil_layouts = if separate_depth_stencil_layouts {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.inner.host_query_reset = if host_query_reset { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.inner.timeline_semaphore = if timeline_semaphore { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.inner.vulkan_memory_model = if vulkan_memory_model { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn vulkan_memory_model_device_scope(mut self, vulkan_memory_model_device_scope: bool) -> Self {
        self.inner.vulkan_memory_model_device_scope = if vulkan_memory_model_device_scope {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.inner.vulkan_memory_model_availability_visibility_chains =
            if vulkan_memory_model_availability_visibility_chains {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_output_viewport_index(mut self, shader_output_viewport_index: bool) -> Self {
        self.inner.shader_output_viewport_index = if shader_output_viewport_index {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_output_layer(mut self, shader_output_layer: bool) -> Self {
        self.inner.shader_output_layer = if shader_output_layer { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn subgroup_broadcast_dynamic_id(mut self, subgroup_broadcast_dynamic_id: bool) -> Self {
        self.inner.subgroup_broadcast_dynamic_id = if subgroup_broadcast_dynamic_id {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceVulkan12FeaturesBuilder {
    type Target = vk::PhysicalDeviceVulkan12Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineCompilerControlCreateInfoAMD {
    type Type = PipelineCompilerControlCreateInfoAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCompilerControlCreateInfoAMDBuilder {
    inner: vk::PipelineCompilerControlCreateInfoAMD,
}
impl PipelineCompilerControlCreateInfoAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compiler_control_flags(mut self, compiler_control_flags: vk::PipelineCompilerControlFlagsAMD) -> Self {
        self.inner.compiler_control_flags = compiler_control_flags;
        self
    }
}
impl Deref for PipelineCompilerControlCreateInfoAMDBuilder {
    type Target = vk::PipelineCompilerControlCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceCoherentMemoryFeaturesAMD {
    type Type = PhysicalDeviceCoherentMemoryFeaturesAMDBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMDBuilder {
    inner: vk::PhysicalDeviceCoherentMemoryFeaturesAMD,
}
impl PhysicalDeviceCoherentMemoryFeaturesAMDBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_coherent_memory(mut self, device_coherent_memory: bool) -> Self {
        self.inner.device_coherent_memory = if device_coherent_memory { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder {
    type Target = vk::PhysicalDeviceCoherentMemoryFeaturesAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::SamplerCustomBorderColorCreateInfoEXT {
    type Type = SamplerCustomBorderColorCreateInfoEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerCustomBorderColorCreateInfoEXTBuilder {
    inner: vk::SamplerCustomBorderColorCreateInfoEXT,
}
impl SamplerCustomBorderColorCreateInfoEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn custom_border_color(mut self, custom_border_color: vk::ClearColorValue) -> Self {
        self.inner.custom_border_color = custom_border_color;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
}
impl Deref for SamplerCustomBorderColorCreateInfoEXTBuilder {
    type Target = vk::SamplerCustomBorderColorCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceCustomBorderColorFeaturesEXT {
    type Type = PhysicalDeviceCustomBorderColorFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceCustomBorderColorFeaturesEXT,
}
impl PhysicalDeviceCustomBorderColorFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn custom_border_colors(mut self, custom_border_colors: bool) -> Self {
        self.inner.custom_border_colors = if custom_border_colors { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn custom_border_color_without_format(mut self, custom_border_color_without_format: bool) -> Self {
        self.inner.custom_border_color_without_format = if custom_border_color_without_format {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceCustomBorderColorFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureGeometryTrianglesDataKHR {
    type Type = AccelerationStructureGeometryTrianglesDataKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryTrianglesDataKHRBuilder {
    inner: vk::AccelerationStructureGeometryTrianglesDataKHR,
}
impl AccelerationStructureGeometryTrianglesDataKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_format(mut self, vertex_format: vk::Format) -> Self {
        self.inner.vertex_format = vertex_format;
        self
    }
    pub fn vertex_data(mut self, vertex_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.vertex_data = vertex_data;
        self
    }
    pub fn vertex_stride(mut self, vertex_stride: vk::DeviceSize) -> Self {
        self.inner.vertex_stride = vertex_stride;
        self
    }
    pub fn max_vertex(mut self, max_vertex: u32) -> Self {
        self.inner.max_vertex = max_vertex;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn index_data(mut self, index_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.index_data = index_data;
        self
    }
    pub fn transform_data(mut self, transform_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.transform_data = transform_data;
        self
    }
}
impl Deref for AccelerationStructureGeometryTrianglesDataKHRBuilder {
    type Target = vk::AccelerationStructureGeometryTrianglesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureGeometryAabbsDataKHR {
    type Type = AccelerationStructureGeometryAabbsDataKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryAabbsDataKHRBuilder {
    inner: vk::AccelerationStructureGeometryAabbsDataKHR,
}
impl AccelerationStructureGeometryAabbsDataKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn data(mut self, data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.data = data;
        self
    }
    pub fn stride(mut self, stride: vk::DeviceSize) -> Self {
        self.inner.stride = stride;
        self
    }
}
impl Deref for AccelerationStructureGeometryAabbsDataKHRBuilder {
    type Target = vk::AccelerationStructureGeometryAabbsDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureGeometryInstancesDataKHR {
    type Type = AccelerationStructureGeometryInstancesDataKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryInstancesDataKHRBuilder {
    inner: vk::AccelerationStructureGeometryInstancesDataKHR,
}
impl AccelerationStructureGeometryInstancesDataKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn array_of_pointers(mut self, array_of_pointers: bool) -> Self {
        self.inner.array_of_pointers = if array_of_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn data(mut self, data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.data = data;
        self
    }
}
impl Deref for AccelerationStructureGeometryInstancesDataKHRBuilder {
    type Target = vk::AccelerationStructureGeometryInstancesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureGeometryKHR {
    type Type = AccelerationStructureGeometryKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryKHRBuilder {
    inner: vk::AccelerationStructureGeometryKHR,
}
impl AccelerationStructureGeometryKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn geometry_type(mut self, geometry_type: vk::GeometryTypeKHR) -> Self {
        self.inner.geometry_type = geometry_type;
        self
    }
    pub fn geometry(mut self, geometry: vk::AccelerationStructureGeometryDataKHR) -> Self {
        self.inner.geometry = geometry;
        self
    }
    pub fn flags(mut self, flags: vk::GeometryFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for AccelerationStructureGeometryKHRBuilder {
    type Target = vk::AccelerationStructureGeometryKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureBuildGeometryInfoKHR {
    type Type = AccelerationStructureBuildGeometryInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureBuildGeometryInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn flags(mut self, flags: vk::BuildAccelerationStructureFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn mode(mut self, mode: vk::BuildAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
    pub fn src_acceleration_structure(
        mut self,
        src_acceleration_structure: Option<vk::AccelerationStructureKHR>,
    ) -> Self {
        self.inner.src_acceleration_structure = src_acceleration_structure;
        self
    }
    pub fn dst_acceleration_structure(
        mut self,
        dst_acceleration_structure: Option<vk::AccelerationStructureKHR>,
    ) -> Self {
        self.inner.dst_acceleration_structure = dst_acceleration_structure;
        self
    }
    pub fn p_geometries(mut self, p_geometries: &'a [vk::AccelerationStructureGeometryKHR]) -> Self {
        self.inner.geometry_count = p_geometries.len() as u32;
        self.inner.p_geometries = p_geometries.as_ptr();
        self
    }
    pub fn pp_geometries(mut self, pp_geometries: &'a [*const vk::AccelerationStructureGeometryKHR]) -> Self {
        self.inner.geometry_count = pp_geometries.len() as u32;
        self.inner.pp_geometries = pp_geometries.as_ptr();
        self
    }
    pub fn scratch_data(mut self, scratch_data: vk::DeviceOrHostAddressKHR) -> Self {
        self.inner.scratch_data = scratch_data;
        self
    }
}
impl<'a> Deref for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureBuildGeometryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureCreateInfoKHR {
    type Type = AccelerationStructureCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureCreateInfoKHRBuilder {
    inner: vk::AccelerationStructureCreateInfoKHR,
}
impl AccelerationStructureCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn create_flags(mut self, create_flags: vk::AccelerationStructureCreateFlagsKHR) -> Self {
        self.inner.create_flags = create_flags;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn device_address(mut self, device_address: vk::DeviceAddress) -> Self {
        self.inner.device_address = device_address;
        self
    }
}
impl Deref for AccelerationStructureCreateInfoKHRBuilder {
    type Target = vk::AccelerationStructureCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureDeviceAddressInfoKHR {
    type Type = AccelerationStructureDeviceAddressInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureDeviceAddressInfoKHRBuilder {
    inner: vk::AccelerationStructureDeviceAddressInfoKHR,
}
impl AccelerationStructureDeviceAddressInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureKHR) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
}
impl Deref for AccelerationStructureDeviceAddressInfoKHRBuilder {
    type Target = vk::AccelerationStructureDeviceAddressInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureVersionInfoKHR {
    type Type = AccelerationStructureVersionInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureVersionInfoKHRBuilder {
    inner: vk::AccelerationStructureVersionInfoKHR,
}
impl AccelerationStructureVersionInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_version_data(mut self, p_version_data: *const u8) -> Self {
        self.inner.p_version_data = p_version_data;
        self
    }
}
impl Deref for AccelerationStructureVersionInfoKHRBuilder {
    type Target = vk::AccelerationStructureVersionInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyAccelerationStructureInfoKHR {
    type Type = CopyAccelerationStructureInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyAccelerationStructureInfoKHRBuilder {
    inner: vk::CopyAccelerationStructureInfoKHR,
}
impl CopyAccelerationStructureInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::AccelerationStructureKHR) -> Self {
        self.inner.src = Some(src);
        self
    }
    pub fn dst(mut self, dst: vk::AccelerationStructureKHR) -> Self {
        self.inner.dst = Some(dst);
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl Deref for CopyAccelerationStructureInfoKHRBuilder {
    type Target = vk::CopyAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyAccelerationStructureToMemoryInfoKHR {
    type Type = CopyAccelerationStructureToMemoryInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyAccelerationStructureToMemoryInfoKHRBuilder {
    inner: vk::CopyAccelerationStructureToMemoryInfoKHR,
}
impl CopyAccelerationStructureToMemoryInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::AccelerationStructureKHR) -> Self {
        self.inner.src = Some(src);
        self
    }
    pub fn dst(mut self, dst: vk::DeviceOrHostAddressKHR) -> Self {
        self.inner.dst = dst;
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl Deref for CopyAccelerationStructureToMemoryInfoKHRBuilder {
    type Target = vk::CopyAccelerationStructureToMemoryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyMemoryToAccelerationStructureInfoKHR {
    type Type = CopyMemoryToAccelerationStructureInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyMemoryToAccelerationStructureInfoKHRBuilder {
    inner: vk::CopyMemoryToAccelerationStructureInfoKHR,
}
impl CopyMemoryToAccelerationStructureInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.src = src;
        self
    }
    pub fn dst(mut self, dst: vk::AccelerationStructureKHR) -> Self {
        self.inner.dst = Some(dst);
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl Deref for CopyMemoryToAccelerationStructureInfoKHRBuilder {
    type Target = vk::CopyMemoryToAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RayTracingPipelineInterfaceCreateInfoKHR {
    type Type = RayTracingPipelineInterfaceCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingPipelineInterfaceCreateInfoKHRBuilder {
    inner: vk::RayTracingPipelineInterfaceCreateInfoKHR,
}
impl RayTracingPipelineInterfaceCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_pipeline_ray_payload_size(mut self, max_pipeline_ray_payload_size: u32) -> Self {
        self.inner.max_pipeline_ray_payload_size = max_pipeline_ray_payload_size;
        self
    }
    pub fn max_pipeline_ray_hit_attribute_size(mut self, max_pipeline_ray_hit_attribute_size: u32) -> Self {
        self.inner.max_pipeline_ray_hit_attribute_size = max_pipeline_ray_hit_attribute_size;
        self
    }
}
impl Deref for RayTracingPipelineInterfaceCreateInfoKHRBuilder {
    type Target = vk::RayTracingPipelineInterfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineLibraryCreateInfoKHR {
    type Type = PipelineLibraryCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineLibraryCreateInfoKHRBuilder<'a> {
    inner: vk::PipelineLibraryCreateInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> PipelineLibraryCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_libraries(mut self, p_libraries: &'a [vk::Pipeline]) -> Self {
        self.inner.library_count = p_libraries.len() as u32;
        self.inner.p_libraries = p_libraries.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineLibraryCreateInfoKHRBuilder<'a> {
    type Target = vk::PipelineLibraryCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    type Type = PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT,
}
impl PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn extended_dynamic_state(mut self, extended_dynamic_state: bool) -> Self {
        self.inner.extended_dynamic_state = if extended_dynamic_state { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::RenderPassTransformBeginInfoQCOM {
    type Type = RenderPassTransformBeginInfoQCOMBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassTransformBeginInfoQCOMBuilder {
    inner: vk::RenderPassTransformBeginInfoQCOM,
}
impl RenderPassTransformBeginInfoQCOMBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
}
impl Deref for RenderPassTransformBeginInfoQCOMBuilder {
    type Target = vk::RenderPassTransformBeginInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CopyCommandTransformInfoQCOM {
    type Type = CopyCommandTransformInfoQCOMBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyCommandTransformInfoQCOMBuilder {
    inner: vk::CopyCommandTransformInfoQCOM,
}
impl CopyCommandTransformInfoQCOMBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
}
impl Deref for CopyCommandTransformInfoQCOMBuilder {
    type Target = vk::CopyCommandTransformInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::CommandBufferInheritanceRenderPassTransformInfoQCOM {
    type Type = CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder {
    inner: vk::CommandBufferInheritanceRenderPassTransformInfoQCOM,
}
impl CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
}
impl Deref for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder {
    type Target = vk::CommandBufferInheritanceRenderPassTransformInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceDiagnosticsConfigFeaturesNV {
    type Type = PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder {
    inner: vk::PhysicalDeviceDiagnosticsConfigFeaturesNV,
}
impl PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn diagnostics_config(mut self, diagnostics_config: bool) -> Self {
        self.inner.diagnostics_config = if diagnostics_config { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceDiagnosticsConfigFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::DeviceDiagnosticsConfigCreateInfoNV {
    type Type = DeviceDiagnosticsConfigCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceDiagnosticsConfigCreateInfoNVBuilder {
    inner: vk::DeviceDiagnosticsConfigCreateInfoNV,
}
impl DeviceDiagnosticsConfigCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceDiagnosticsConfigFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl Deref for DeviceDiagnosticsConfigCreateInfoNVBuilder {
    type Target = vk::DeviceDiagnosticsConfigCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceRobustness2FeaturesEXT {
    type Type = PhysicalDeviceRobustness2FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRobustness2FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceRobustness2FeaturesEXT,
}
impl PhysicalDeviceRobustness2FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn robust_buffer_access2(mut self, robust_buffer_access2: bool) -> Self {
        self.inner.robust_buffer_access2 = if robust_buffer_access2 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn robust_image_access2(mut self, robust_image_access2: bool) -> Self {
        self.inner.robust_image_access2 = if robust_image_access2 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn null_descriptor(mut self, null_descriptor: bool) -> Self {
        self.inner.null_descriptor = if null_descriptor { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceRobustness2FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceRobustness2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceImageRobustnessFeaturesEXT {
    type Type = PhysicalDeviceImageRobustnessFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageRobustnessFeaturesEXTBuilder {
    inner: vk::PhysicalDeviceImageRobustnessFeaturesEXT,
}
impl PhysicalDeviceImageRobustnessFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn robust_image_access(mut self, robust_image_access: bool) -> Self {
        self.inner.robust_image_access = if robust_image_access { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDeviceImageRobustnessFeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceImageRobustnessFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePortabilitySubsetFeaturesKHR {
    type Type = PhysicalDevicePortabilitySubsetFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHRBuilder {
    inner: vk::PhysicalDevicePortabilitySubsetFeaturesKHR,
}
impl PhysicalDevicePortabilitySubsetFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn constant_alpha_color_blend_factors(mut self, constant_alpha_color_blend_factors: bool) -> Self {
        self.inner.constant_alpha_color_blend_factors = if constant_alpha_color_blend_factors {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn events(mut self, events: bool) -> Self {
        self.inner.events = if events { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn image_view_format_reinterpretation(mut self, image_view_format_reinterpretation: bool) -> Self {
        self.inner.image_view_format_reinterpretation = if image_view_format_reinterpretation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn image_view_format_swizzle(mut self, image_view_format_swizzle: bool) -> Self {
        self.inner.image_view_format_swizzle = if image_view_format_swizzle { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn image_view2_d_on3_d_image(mut self, image_view2_d_on3_d_image: bool) -> Self {
        self.inner.image_view2_d_on3_d_image = if image_view2_d_on3_d_image { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multisample_array_image(mut self, multisample_array_image: bool) -> Self {
        self.inner.multisample_array_image = if multisample_array_image { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn mutable_comparison_samplers(mut self, mutable_comparison_samplers: bool) -> Self {
        self.inner.mutable_comparison_samplers = if mutable_comparison_samplers {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn point_polygons(mut self, point_polygons: bool) -> Self {
        self.inner.point_polygons = if point_polygons { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sampler_mip_lod_bias(mut self, sampler_mip_lod_bias: bool) -> Self {
        self.inner.sampler_mip_lod_bias = if sampler_mip_lod_bias { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn separate_stencil_mask_ref(mut self, separate_stencil_mask_ref: bool) -> Self {
        self.inner.separate_stencil_mask_ref = if separate_stencil_mask_ref { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_sample_rate_interpolation_functions(
        mut self,
        shader_sample_rate_interpolation_functions: bool,
    ) -> Self {
        self.inner.shader_sample_rate_interpolation_functions = if shader_sample_rate_interpolation_functions {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn tessellation_isolines(mut self, tessellation_isolines: bool) -> Self {
        self.inner.tessellation_isolines = if tessellation_isolines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn tessellation_point_mode(mut self, tessellation_point_mode: bool) -> Self {
        self.inner.tessellation_point_mode = if tessellation_point_mode { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn triangle_fans(mut self, triangle_fans: bool) -> Self {
        self.inner.triangle_fans = if triangle_fans { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn vertex_attribute_access_beyond_stride(mut self, vertex_attribute_access_beyond_stride: bool) -> Self {
        self.inner.vertex_attribute_access_beyond_stride = if vertex_attribute_access_beyond_stride {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDevicePortabilitySubsetFeaturesKHRBuilder {
    type Target = vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevicePortabilitySubsetPropertiesKHR {
    type Type = PhysicalDevicePortabilitySubsetPropertiesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHRBuilder {
    inner: vk::PhysicalDevicePortabilitySubsetPropertiesKHR,
}
impl PhysicalDevicePortabilitySubsetPropertiesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn min_vertex_input_binding_stride_alignment(mut self, min_vertex_input_binding_stride_alignment: u32) -> Self {
        self.inner.min_vertex_input_binding_stride_alignment = min_vertex_input_binding_stride_alignment;
        self
    }
}
impl Deref for PhysicalDevicePortabilitySubsetPropertiesKHRBuilder {
    type Target = vk::PhysicalDevicePortabilitySubsetPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDevice4444FormatsFeaturesEXT {
    type Type = PhysicalDevice4444FormatsFeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevice4444FormatsFeaturesEXTBuilder {
    inner: vk::PhysicalDevice4444FormatsFeaturesEXT,
}
impl PhysicalDevice4444FormatsFeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format_a4r4g4b4(mut self, format_a4r4g4b4: bool) -> Self {
        self.inner.format_a4r4g4b4 = if format_a4r4g4b4 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn format_a4b4g4r4(mut self, format_a4b4g4r4: bool) -> Self {
        self.inner.format_a4b4g4r4 = if format_a4b4g4r4 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl Deref for PhysicalDevice4444FormatsFeaturesEXTBuilder {
    type Target = vk::PhysicalDevice4444FormatsFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferCopy2KHR {
    type Type = BufferCopy2KHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCopy2KHRBuilder {
    inner: vk::BufferCopy2KHR,
}
impl BufferCopy2KHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_offset(mut self, src_offset: vk::DeviceSize) -> Self {
        self.inner.src_offset = src_offset;
        self
    }
    pub fn dst_offset(mut self, dst_offset: vk::DeviceSize) -> Self {
        self.inner.dst_offset = dst_offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl Deref for BufferCopy2KHRBuilder {
    type Target = vk::BufferCopy2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageCopy2KHR {
    type Type = ImageCopy2KHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageCopy2KHRBuilder {
    inner: vk::ImageCopy2KHR,
}
impl ImageCopy2KHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subresource(mut self, src_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.src_subresource = src_subresource;
        self
    }
    pub fn src_offset(mut self, src_offset: vk::Offset3D) -> Self {
        self.inner.src_offset = src_offset;
        self
    }
    pub fn dst_subresource(mut self, dst_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.dst_subresource = dst_subresource;
        self
    }
    pub fn dst_offset(mut self, dst_offset: vk::Offset3D) -> Self {
        self.inner.dst_offset = dst_offset;
        self
    }
    pub fn extent(mut self, extent: vk::Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }
}
impl Deref for ImageCopy2KHRBuilder {
    type Target = vk::ImageCopy2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageBlit2KHR {
    type Type = ImageBlit2KHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageBlit2KHRBuilder {
    inner: vk::ImageBlit2KHR,
}
impl ImageBlit2KHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subresource(mut self, src_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.src_subresource = src_subresource;
        self
    }
    pub fn dst_subresource(mut self, dst_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.dst_subresource = dst_subresource;
        self
    }
}
impl Deref for ImageBlit2KHRBuilder {
    type Target = vk::ImageBlit2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::BufferImageCopy2KHR {
    type Type = BufferImageCopy2KHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferImageCopy2KHRBuilder {
    inner: vk::BufferImageCopy2KHR,
}
impl BufferImageCopy2KHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer_offset(mut self, buffer_offset: vk::DeviceSize) -> Self {
        self.inner.buffer_offset = buffer_offset;
        self
    }
    pub fn buffer_row_length(mut self, buffer_row_length: u32) -> Self {
        self.inner.buffer_row_length = buffer_row_length;
        self
    }
    pub fn buffer_image_height(mut self, buffer_image_height: u32) -> Self {
        self.inner.buffer_image_height = buffer_image_height;
        self
    }
    pub fn image_subresource(mut self, image_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.image_subresource = image_subresource;
        self
    }
    pub fn image_offset(mut self, image_offset: vk::Offset3D) -> Self {
        self.inner.image_offset = image_offset;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent3D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
}
impl Deref for BufferImageCopy2KHRBuilder {
    type Target = vk::BufferImageCopy2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::ImageResolve2KHR {
    type Type = ImageResolve2KHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageResolve2KHRBuilder {
    inner: vk::ImageResolve2KHR,
}
impl ImageResolve2KHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subresource(mut self, src_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.src_subresource = src_subresource;
        self
    }
    pub fn src_offset(mut self, src_offset: vk::Offset3D) -> Self {
        self.inner.src_offset = src_offset;
        self
    }
    pub fn dst_subresource(mut self, dst_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.dst_subresource = dst_subresource;
        self
    }
    pub fn dst_offset(mut self, dst_offset: vk::Offset3D) -> Self {
        self.inner.dst_offset = dst_offset;
        self
    }
    pub fn extent(mut self, extent: vk::Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }
}
impl Deref for ImageResolve2KHRBuilder {
    type Target = vk::ImageResolve2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyBufferInfo2KHR {
    type Type = CopyBufferInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyBufferInfo2KHRBuilder<'a> {
    inner: vk::CopyBufferInfo2KHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CopyBufferInfo2KHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_buffer(mut self, src_buffer: vk::Buffer) -> Self {
        self.inner.src_buffer = Some(src_buffer);
        self
    }
    pub fn dst_buffer(mut self, dst_buffer: vk::Buffer) -> Self {
        self.inner.dst_buffer = Some(dst_buffer);
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferCopy2KHR]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyBufferInfo2KHRBuilder<'a> {
    type Target = vk::CopyBufferInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyImageInfo2KHR {
    type Type = CopyImageInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyImageInfo2KHRBuilder<'a> {
    inner: vk::CopyImageInfo2KHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CopyImageInfo2KHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = Some(src_image);
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = Some(dst_image);
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageCopy2KHR]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyImageInfo2KHRBuilder<'a> {
    type Target = vk::CopyImageInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BlitImageInfo2KHR {
    type Type = BlitImageInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BlitImageInfo2KHRBuilder<'a> {
    inner: vk::BlitImageInfo2KHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> BlitImageInfo2KHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = Some(src_image);
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = Some(dst_image);
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageBlit2KHR]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
    pub fn filter(mut self, filter: vk::Filter) -> Self {
        self.inner.filter = filter;
        self
    }
}
impl<'a> Deref for BlitImageInfo2KHRBuilder<'a> {
    type Target = vk::BlitImageInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyBufferToImageInfo2KHR {
    type Type = CopyBufferToImageInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyBufferToImageInfo2KHRBuilder<'a> {
    inner: vk::CopyBufferToImageInfo2KHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CopyBufferToImageInfo2KHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_buffer(mut self, src_buffer: vk::Buffer) -> Self {
        self.inner.src_buffer = Some(src_buffer);
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = Some(dst_image);
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferImageCopy2KHR]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyBufferToImageInfo2KHRBuilder<'a> {
    type Target = vk::CopyBufferToImageInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyImageToBufferInfo2KHR {
    type Type = CopyImageToBufferInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyImageToBufferInfo2KHRBuilder<'a> {
    inner: vk::CopyImageToBufferInfo2KHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> CopyImageToBufferInfo2KHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = Some(src_image);
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_buffer(mut self, dst_buffer: vk::Buffer) -> Self {
        self.inner.dst_buffer = Some(dst_buffer);
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferImageCopy2KHR]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyImageToBufferInfo2KHRBuilder<'a> {
    type Target = vk::CopyImageToBufferInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ResolveImageInfo2KHR {
    type Type = ResolveImageInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ResolveImageInfo2KHRBuilder<'a> {
    inner: vk::ResolveImageInfo2KHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> ResolveImageInfo2KHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = Some(src_image);
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = Some(dst_image);
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageResolve2KHR]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for ResolveImageInfo2KHRBuilder<'a> {
    type Target = vk::ResolveImageInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    type Type = PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder {
    inner: vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT,
}
impl PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_image_int64_atomics(mut self, shader_image_int64_atomics: bool) -> Self {
        self.inner.shader_image_int64_atomics = if shader_image_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn sparse_image_int64_atomics(mut self, sparse_image_int64_atomics: bool) -> Self {
        self.inner.sparse_image_int64_atomics = if sparse_image_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderImageAtomicInt64FeaturesEXTBuilder {
    type Target = vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FragmentShadingRateAttachmentInfoKHR {
    type Type = FragmentShadingRateAttachmentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    inner: vk::FragmentShadingRateAttachmentInfoKHR,
    phantom: PhantomData<&'a vk::Never>,
}
impl<'a> FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_fragment_shading_rate_attachment(
        mut self,
        p_fragment_shading_rate_attachment: &'a vk::AttachmentReference2,
    ) -> Self {
        self.inner.p_fragment_shading_rate_attachment = p_fragment_shading_rate_attachment;
        self
    }
    pub fn shading_rate_attachment_texel_size(mut self, shading_rate_attachment_texel_size: vk::Extent2D) -> Self {
        self.inner.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
        self
    }
}
impl<'a> Deref for FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    type Target = vk::FragmentShadingRateAttachmentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineFragmentShadingRateStateCreateInfoKHR {
    type Type = PipelineFragmentShadingRateStateCreateInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHRBuilder {
    inner: vk::PipelineFragmentShadingRateStateCreateInfoKHR,
}
impl PipelineFragmentShadingRateStateCreateInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_size(mut self, fragment_size: vk::Extent2D) -> Self {
        self.inner.fragment_size = fragment_size;
        self
    }
}
impl Deref for PipelineFragmentShadingRateStateCreateInfoKHRBuilder {
    type Target = vk::PipelineFragmentShadingRateStateCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceFragmentShadingRateFeaturesKHR {
    type Type = PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceFragmentShadingRateFeaturesKHR,
}
impl PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_fragment_shading_rate(mut self, pipeline_fragment_shading_rate: bool) -> Self {
        self.inner.pipeline_fragment_shading_rate = if pipeline_fragment_shading_rate {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn primitive_fragment_shading_rate(mut self, primitive_fragment_shading_rate: bool) -> Self {
        self.inner.primitive_fragment_shading_rate = if primitive_fragment_shading_rate {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn attachment_fragment_shading_rate(mut self, attachment_fragment_shading_rate: bool) -> Self {
        self.inner.attachment_fragment_shading_rate = if attachment_fragment_shading_rate {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentShadingRateFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceFragmentShadingRateFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceShaderTerminateInvocationFeaturesKHR {
    type Type = PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder {
    inner: vk::PhysicalDeviceShaderTerminateInvocationFeaturesKHR,
}
impl PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_terminate_invocation(mut self, shader_terminate_invocation: bool) -> Self {
        self.inner.shader_terminate_invocation = if shader_terminate_invocation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceShaderTerminateInvocationFeaturesKHRBuilder {
    type Target = vk::PhysicalDeviceShaderTerminateInvocationFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    type Type = PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder {
    inner: vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV,
}
impl PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_shading_rate_enums(mut self, fragment_shading_rate_enums: bool) -> Self {
        self.inner.fragment_shading_rate_enums = if fragment_shading_rate_enums {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn supersample_fragment_shading_rates(mut self, supersample_fragment_shading_rates: bool) -> Self {
        self.inner.supersample_fragment_shading_rates = if supersample_fragment_shading_rates {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn no_invocation_fragment_shading_rates(mut self, no_invocation_fragment_shading_rates: bool) -> Self {
        self.inner.no_invocation_fragment_shading_rates = if no_invocation_fragment_shading_rates {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl Deref for PhysicalDeviceFragmentShadingRateEnumsFeaturesNVBuilder {
    type Target = vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    type Type = PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder {
    inner: vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV,
}
impl PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_fragment_shading_rate_invocation_count(
        mut self,
        max_fragment_shading_rate_invocation_count: vk::SampleCountFlags,
    ) -> Self {
        self.inner.max_fragment_shading_rate_invocation_count = max_fragment_shading_rate_invocation_count;
        self
    }
}
impl Deref for PhysicalDeviceFragmentShadingRateEnumsPropertiesNVBuilder {
    type Target = vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::PipelineFragmentShadingRateEnumStateCreateInfoNV {
    type Type = PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder {
    inner: vk::PipelineFragmentShadingRateEnumStateCreateInfoNV,
}
impl PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_type(mut self, shading_rate_type: vk::FragmentShadingRateTypeNV) -> Self {
        self.inner.shading_rate_type = shading_rate_type;
        self
    }
    pub fn shading_rate(mut self, shading_rate: vk::FragmentShadingRateNV) -> Self {
        self.inner.shading_rate = shading_rate;
        self
    }
}
impl Deref for PipelineFragmentShadingRateEnumStateCreateInfoNVBuilder {
    type Target = vk::PipelineFragmentShadingRateEnumStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Builder<'_> for vk::AccelerationStructureBuildSizesInfoKHR {
    type Type = AccelerationStructureBuildSizesInfoKHRBuilder;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureBuildSizesInfoKHRBuilder {
    inner: vk::AccelerationStructureBuildSizesInfoKHR,
}
impl AccelerationStructureBuildSizesInfoKHRBuilder {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure_size(mut self, acceleration_structure_size: vk::DeviceSize) -> Self {
        self.inner.acceleration_structure_size = acceleration_structure_size;
        self
    }
    pub fn update_scratch_size(mut self, update_scratch_size: vk::DeviceSize) -> Self {
        self.inner.update_scratch_size = update_scratch_size;
        self
    }
    pub fn build_scratch_size(mut self, build_scratch_size: vk::DeviceSize) -> Self {
        self.inner.build_scratch_size = build_scratch_size;
        self
    }
}
impl Deref for AccelerationStructureBuildSizesInfoKHRBuilder {
    type Target = vk::AccelerationStructureBuildSizesInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
