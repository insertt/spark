//! Generated from vk.xml with `VK_HEADER_VERSION` 175
#![allow(
    clippy::too_many_arguments,
    clippy::trivially_copy_pass_by_ref,
    clippy::missing_safety_doc
)]

pub mod builder;
pub mod vk;

use lazy_static::lazy_static;
use shared_library::dynamic_library::DynamicLibrary;
use std::ffi::CStr;
use std::mem;
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};
use std::path::Path;
use std::ptr;
use std::result;
use std::slice;

#[doc(no_inline)]
pub use self::builder::*;

pub type Result<T> = result::Result<T, vk::Result>;

struct Lib {
    pub lib: DynamicLibrary,
    pub fp_get_instance_proc_addr: vk::FnGetInstanceProcAddr,
}

#[derive(Debug, Clone)]
pub enum LoaderError {
    DynamicLibrary(String),
    MissingSymbol(String),
    Vulkan(vk::Result),
}

impl From<vk::Result> for LoaderError {
    fn from(err: vk::Result) -> Self {
        LoaderError::Vulkan(err)
    }
}

pub type LoaderResult<T> = result::Result<T, LoaderError>;

#[cfg(target_os = "linux")]
const DL_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "windows")]
const DL_PATH: &str = "vulkan-1.dll";

#[cfg(target_os = "android")]
const DL_PATH: &str = "libvulkan.so";

impl Lib {
    pub fn new() -> LoaderResult<Self> {
        match DynamicLibrary::open(Some(&Path::new(&DL_PATH))) {
            Ok(lib) => match unsafe {
                lib.symbol("vkGetInstanceProcAddr")
                    .map(|f: *mut c_void| mem::transmute(f))
            } {
                Ok(fp_get_instance_proc_addr) => Ok(Self {
                    lib,
                    fp_get_instance_proc_addr,
                }),
                Err(s) => Err(LoaderError::MissingSymbol(s)),
            },
            Err(s) => Err(LoaderError::DynamicLibrary(s)),
        }
    }

    pub unsafe fn get_instance_proc_addr(&self, name: &CStr) -> Option<vk::FnVoidFunction> {
        (self.fp_get_instance_proc_addr)(None, name.as_ptr())
    }
}

lazy_static! {
    static ref LIB: LoaderResult<Lib> = Lib::new();
}
#[derive(Copy, Clone)]
pub struct Loader {
    pub fp_create_instance: Option<vk::FnCreateInstance>,
    pub fp_get_instance_proc_addr: Option<vk::FnGetInstanceProcAddr>,
    pub fp_enumerate_instance_version: Option<vk::FnEnumerateInstanceVersion>,
    pub fp_enumerate_instance_layer_properties: Option<vk::FnEnumerateInstanceLayerProperties>,
    pub fp_enumerate_instance_extension_properties: Option<vk::FnEnumerateInstanceExtensionProperties>,
}
impl Loader {
    pub fn new() -> LoaderResult<Self> {
        let lib = LIB.as_ref().map_err(|e| e.clone())?;
        unsafe {
            let f = |name: &CStr| lib.get_instance_proc_addr(name);
            Ok(Self {
                fp_create_instance: {
                    let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateInstance\0"));
                    fp.map(|f| mem::transmute(f))
                },
                fp_get_instance_proc_addr: Some(lib.fp_get_instance_proc_addr),
                fp_enumerate_instance_version: {
                    let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkEnumerateInstanceVersion\0"));
                    fp.map(|f| mem::transmute(f))
                },
                fp_enumerate_instance_layer_properties: {
                    let fp = f(CStr::from_bytes_with_nul_unchecked(
                        b"vkEnumerateInstanceLayerProperties\0",
                    ));
                    fp.map(|f| mem::transmute(f))
                },
                fp_enumerate_instance_extension_properties: {
                    let fp = f(CStr::from_bytes_with_nul_unchecked(
                        b"vkEnumerateInstanceExtensionProperties\0",
                    ));
                    fp.map(|f| mem::transmute(f))
                },
            })
        }
    }
    pub unsafe fn create_instance(
        &self,
        p_create_info: &vk::InstanceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> result::Result<Instance, LoaderError> {
        let fp = self.fp_create_instance.expect("vkCreateInstance is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(p_create_info, p_allocator.map_or(ptr::null(), |r| r), res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
        .map_err(LoaderError::Vulkan)
        .and_then(|r| Instance::load(&self, r, p_create_info))
    }
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Option<vk::Instance>,
        p_name: &CStr,
    ) -> Option<vk::FnVoidFunction> {
        let fp = self
            .fp_get_instance_proc_addr
            .expect("vkGetInstanceProcAddr is not loaded");
        (fp)(instance, p_name.as_ptr())
    }
    pub unsafe fn enumerate_instance_version(&self) -> Result<vk::Version> {
        if let Some(fp) = self.fp_enumerate_instance_version {
            let mut res = MaybeUninit::<_>::uninit();
            let err = (fp)(res.as_mut_ptr());
            match err {
                vk::Result::SUCCESS => Ok(res.assume_init()),
                _ => Err(err),
            }
        } else {
            Ok(vk::Version::default())
        }
    }
    pub unsafe fn enumerate_instance_layer_properties_to_vec(&self) -> Result<Vec<vk::LayerProperties>> {
        let fp = self
            .fp_enumerate_instance_layer_properties
            .expect("vkEnumerateInstanceLayerProperties is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(&mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn enumerate_instance_extension_properties_to_vec(
        &self,
        p_layer_name: Option<&CStr>,
    ) -> Result<Vec<vk::ExtensionProperties>> {
        let fp = self
            .fp_enumerate_instance_extension_properties
            .expect("vkEnumerateInstanceExtensionProperties is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(
            p_layer_name.map_or(ptr::null(), |s| s.as_ptr()),
            len.as_mut_ptr(),
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(
            p_layer_name.map_or(ptr::null(), |s| s.as_ptr()),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct InstanceExtensions {
    pub core_version: vk::Version,
    pub khr_surface: bool,
    pub khr_display: bool,
    pub khr_xlib_surface: bool,
    pub khr_xcb_surface: bool,
    pub khr_wayland_surface: bool,
    pub khr_android_surface: bool,
    pub khr_win32_surface: bool,
    pub ext_debug_report: bool,
    pub nv_external_memory_capabilities: bool,
    pub khr_get_physical_device_properties2: bool,
    pub ext_validation_flags: bool,
    pub nn_vi_surface: bool,
    pub khr_device_group_creation: bool,
    pub khr_external_memory_capabilities: bool,
    pub khr_external_semaphore_capabilities: bool,
    pub ext_direct_mode_display: bool,
    pub ext_acquire_xlib_display: bool,
    pub ext_display_surface_counter: bool,
    pub ext_swapchain_colorspace: bool,
    pub khr_external_fence_capabilities: bool,
    pub khr_get_surface_capabilities2: bool,
    pub khr_get_display_properties2: bool,
    pub mvk_ios_surface: bool,
    pub mvk_macos_surface: bool,
    pub ext_debug_utils: bool,
    pub fuchsia_imagepipe_surface: bool,
    pub ext_metal_surface: bool,
    pub khr_surface_protected_capabilities: bool,
    pub ext_validation_features: bool,
    pub ext_headless_surface: bool,
    pub ext_directfb_surface: bool,
}
impl InstanceExtensions {
    fn enable_by_name(&mut self, name: &CStr) {
        match name.to_bytes() {
            b"VK_KHR_surface" => self.khr_surface = true,
            b"VK_KHR_display" => self.khr_display = true,
            b"VK_KHR_xlib_surface" => self.khr_xlib_surface = true,
            b"VK_KHR_xcb_surface" => self.khr_xcb_surface = true,
            b"VK_KHR_wayland_surface" => self.khr_wayland_surface = true,
            b"VK_KHR_android_surface" => self.khr_android_surface = true,
            b"VK_KHR_win32_surface" => self.khr_win32_surface = true,
            b"VK_EXT_debug_report" => self.ext_debug_report = true,
            b"VK_NV_external_memory_capabilities" => self.nv_external_memory_capabilities = true,
            b"VK_KHR_get_physical_device_properties2" => self.khr_get_physical_device_properties2 = true,
            b"VK_EXT_validation_flags" => self.ext_validation_flags = true,
            b"VK_NN_vi_surface" => self.nn_vi_surface = true,
            b"VK_KHR_device_group_creation" => self.khr_device_group_creation = true,
            b"VK_KHR_external_memory_capabilities" => self.khr_external_memory_capabilities = true,
            b"VK_KHR_external_semaphore_capabilities" => self.khr_external_semaphore_capabilities = true,
            b"VK_EXT_direct_mode_display" => self.ext_direct_mode_display = true,
            b"VK_EXT_acquire_xlib_display" => self.ext_acquire_xlib_display = true,
            b"VK_EXT_display_surface_counter" => self.ext_display_surface_counter = true,
            b"VK_EXT_swapchain_colorspace" => self.ext_swapchain_colorspace = true,
            b"VK_KHR_external_fence_capabilities" => self.khr_external_fence_capabilities = true,
            b"VK_KHR_get_surface_capabilities2" => self.khr_get_surface_capabilities2 = true,
            b"VK_KHR_get_display_properties2" => self.khr_get_display_properties2 = true,
            b"VK_MVK_ios_surface" => self.mvk_ios_surface = true,
            b"VK_MVK_macos_surface" => self.mvk_macos_surface = true,
            b"VK_EXT_debug_utils" => self.ext_debug_utils = true,
            b"VK_FUCHSIA_imagepipe_surface" => self.fuchsia_imagepipe_surface = true,
            b"VK_EXT_metal_surface" => self.ext_metal_surface = true,
            b"VK_KHR_surface_protected_capabilities" => self.khr_surface_protected_capabilities = true,
            b"VK_EXT_validation_features" => self.ext_validation_features = true,
            b"VK_EXT_headless_surface" => self.ext_headless_surface = true,
            b"VK_EXT_directfb_surface" => self.ext_directfb_surface = true,
            _ => {}
        }
    }
    pub fn new(core_version: vk::Version) -> Self {
        Self {
            core_version,
            khr_surface: false,
            khr_display: false,
            khr_xlib_surface: false,
            khr_xcb_surface: false,
            khr_wayland_surface: false,
            khr_android_surface: false,
            khr_win32_surface: false,
            ext_debug_report: false,
            nv_external_memory_capabilities: false,
            khr_get_physical_device_properties2: false,
            ext_validation_flags: false,
            nn_vi_surface: false,
            khr_device_group_creation: false,
            khr_external_memory_capabilities: false,
            khr_external_semaphore_capabilities: false,
            ext_direct_mode_display: false,
            ext_acquire_xlib_display: false,
            ext_display_surface_counter: false,
            ext_swapchain_colorspace: false,
            khr_external_fence_capabilities: false,
            khr_get_surface_capabilities2: false,
            khr_get_display_properties2: false,
            mvk_ios_surface: false,
            mvk_macos_surface: false,
            ext_debug_utils: false,
            fuchsia_imagepipe_surface: false,
            ext_metal_surface: false,
            khr_surface_protected_capabilities: false,
            ext_validation_features: false,
            ext_headless_surface: false,
            ext_directfb_surface: false,
        }
    }
    pub fn from_properties(core_version: vk::Version, properties: &[vk::ExtensionProperties]) -> Self {
        let mut ext = Self::new(core_version);
        for ep in properties.iter() {
            if ep.extension_name.iter().any(|&c| c == 0) {
                let name = unsafe { CStr::from_ptr(ep.extension_name.as_ptr()) };
                ext.enable_by_name(name);
            }
        }
        ext
    }
    pub fn supports_khr_surface(&self) -> bool {
        self.khr_surface
    }
    pub fn enable_khr_surface(&mut self) {
        self.khr_surface = true;
    }
    pub fn supports_khr_swapchain(&self) -> bool {
        self.supports_khr_surface()
    }
    pub fn enable_khr_swapchain(&mut self) {
        self.enable_khr_surface();
    }
    pub fn supports_khr_display(&self) -> bool {
        self.khr_display && self.supports_khr_surface()
    }
    pub fn enable_khr_display(&mut self) {
        self.khr_display = true;
        self.enable_khr_surface();
    }
    pub fn supports_khr_display_swapchain(&self) -> bool {
        self.supports_khr_display() && self.supports_khr_surface()
    }
    pub fn enable_khr_display_swapchain(&mut self) {
        self.enable_khr_display();
        self.enable_khr_surface();
    }
    pub fn supports_khr_xlib_surface(&self) -> bool {
        self.khr_xlib_surface && self.supports_khr_surface()
    }
    pub fn enable_khr_xlib_surface(&mut self) {
        self.khr_xlib_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_khr_xcb_surface(&self) -> bool {
        self.khr_xcb_surface && self.supports_khr_surface()
    }
    pub fn enable_khr_xcb_surface(&mut self) {
        self.khr_xcb_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_khr_wayland_surface(&self) -> bool {
        self.khr_wayland_surface && self.supports_khr_surface()
    }
    pub fn enable_khr_wayland_surface(&mut self) {
        self.khr_wayland_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_khr_android_surface(&self) -> bool {
        self.khr_android_surface && self.supports_khr_surface()
    }
    pub fn enable_khr_android_surface(&mut self) {
        self.khr_android_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_khr_win32_surface(&self) -> bool {
        self.khr_win32_surface && self.supports_khr_surface()
    }
    pub fn enable_khr_win32_surface(&mut self) {
        self.khr_win32_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_ext_debug_report(&self) -> bool {
        self.ext_debug_report
    }
    pub fn enable_ext_debug_report(&mut self) {
        self.ext_debug_report = true;
    }
    pub fn supports_ext_debug_marker(&self) -> bool {
        self.supports_ext_debug_report()
    }
    pub fn enable_ext_debug_marker(&mut self) {
        self.enable_ext_debug_report();
    }
    pub fn supports_ext_transform_feedback(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_transform_feedback(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_amd_texture_gather_bias_lod(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_amd_texture_gather_bias_lod(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_corner_sampled_image(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_corner_sampled_image(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_multiview(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_multiview(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_external_memory_capabilities(&self) -> bool {
        self.nv_external_memory_capabilities
    }
    pub fn enable_nv_external_memory_capabilities(&mut self) {
        self.nv_external_memory_capabilities = true;
    }
    pub fn supports_nv_external_memory(&self) -> bool {
        self.supports_nv_external_memory_capabilities()
    }
    pub fn enable_nv_external_memory(&mut self) {
        self.enable_nv_external_memory_capabilities();
    }
    pub fn supports_nv_external_memory_win32(&self) -> bool {
        self.supports_nv_external_memory_capabilities()
    }
    pub fn enable_nv_external_memory_win32(&mut self) {
        self.enable_nv_external_memory_capabilities();
    }
    pub fn supports_nv_win32_keyed_mutex(&self) -> bool {
        self.supports_nv_external_memory_capabilities()
    }
    pub fn enable_nv_win32_keyed_mutex(&mut self) {
        self.enable_nv_external_memory_capabilities();
    }
    pub fn supports_khr_get_physical_device_properties2(&self) -> bool {
        self.khr_get_physical_device_properties2 || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_get_physical_device_properties2(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_get_physical_device_properties2 = true;
        }
    }
    pub fn supports_khr_device_group(&self) -> bool {
        self.supports_khr_device_group_creation()
    }
    pub fn enable_khr_device_group(&mut self) {
        self.enable_khr_device_group_creation();
    }
    pub fn supports_ext_validation_flags(&self) -> bool {
        self.ext_validation_flags
    }
    pub fn enable_ext_validation_flags(&mut self) {
        self.ext_validation_flags = true;
    }
    pub fn supports_nn_vi_surface(&self) -> bool {
        self.nn_vi_surface && self.supports_khr_surface()
    }
    pub fn enable_nn_vi_surface(&mut self) {
        self.nn_vi_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_ext_texture_compression_astc_hdr(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_texture_compression_astc_hdr(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_astc_decode_mode(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_astc_decode_mode(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_device_group_creation(&self) -> bool {
        self.khr_device_group_creation || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_device_group_creation(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_device_group_creation = true;
        }
    }
    pub fn supports_khr_external_memory_capabilities(&self) -> bool {
        (self.khr_external_memory_capabilities || self.core_version >= vk::Version::from_raw_parts(1, 1, 0))
            && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_memory_capabilities(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_external_memory_capabilities = true;
        }
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_memory(&self) -> bool {
        self.supports_khr_external_memory_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_memory(&mut self) {
        self.enable_khr_external_memory_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_memory_win32(&self) -> bool {
        self.supports_khr_external_memory_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_memory_win32(&mut self) {
        self.enable_khr_external_memory_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_memory_fd(&self) -> bool {
        self.supports_khr_external_memory_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_memory_fd(&mut self) {
        self.enable_khr_external_memory_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_win32_keyed_mutex(&self) -> bool {
        self.supports_khr_external_memory_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_win32_keyed_mutex(&mut self) {
        self.enable_khr_external_memory_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_semaphore_capabilities(&self) -> bool {
        (self.khr_external_semaphore_capabilities || self.core_version >= vk::Version::from_raw_parts(1, 1, 0))
            && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_semaphore_capabilities(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_external_semaphore_capabilities = true;
        }
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_semaphore(&self) -> bool {
        self.supports_khr_external_semaphore_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_semaphore(&mut self) {
        self.enable_khr_external_semaphore_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_semaphore_win32(&self) -> bool {
        self.supports_khr_external_semaphore_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_semaphore_win32(&mut self) {
        self.enable_khr_external_semaphore_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_semaphore_fd(&self) -> bool {
        self.supports_khr_external_semaphore_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_semaphore_fd(&mut self) {
        self.enable_khr_external_semaphore_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_push_descriptor(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_push_descriptor(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_shader_float16_int8(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_shader_float16_int8(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_16bit_storage(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_16bit_storage(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_incremental_present(&self) -> bool {
        self.supports_khr_surface()
    }
    pub fn enable_khr_incremental_present(&mut self) {
        self.enable_khr_surface();
    }
    pub fn supports_ext_direct_mode_display(&self) -> bool {
        self.ext_direct_mode_display && self.supports_khr_display() && self.supports_khr_surface()
    }
    pub fn enable_ext_direct_mode_display(&mut self) {
        self.ext_direct_mode_display = true;
        self.enable_khr_display();
        self.enable_khr_surface();
    }
    pub fn supports_ext_acquire_xlib_display(&self) -> bool {
        self.ext_acquire_xlib_display
            && self.supports_ext_direct_mode_display()
            && self.supports_khr_display()
            && self.supports_khr_surface()
    }
    pub fn enable_ext_acquire_xlib_display(&mut self) {
        self.ext_acquire_xlib_display = true;
        self.enable_ext_direct_mode_display();
        self.enable_khr_display();
        self.enable_khr_surface();
    }
    pub fn supports_ext_display_surface_counter(&self) -> bool {
        self.ext_display_surface_counter && self.supports_khr_display() && self.supports_khr_surface()
    }
    pub fn enable_ext_display_surface_counter(&mut self) {
        self.ext_display_surface_counter = true;
        self.enable_khr_display();
        self.enable_khr_surface();
    }
    pub fn supports_ext_display_control(&self) -> bool {
        self.supports_ext_display_surface_counter() && self.supports_khr_display() && self.supports_khr_surface()
    }
    pub fn enable_ext_display_control(&mut self) {
        self.enable_ext_display_surface_counter();
        self.enable_khr_display();
        self.enable_khr_surface();
    }
    pub fn supports_google_display_timing(&self) -> bool {
        self.supports_khr_surface()
    }
    pub fn enable_google_display_timing(&mut self) {
        self.enable_khr_surface();
    }
    pub fn supports_nvx_multiview_per_view_attributes(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nvx_multiview_per_view_attributes(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_discard_rectangles(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_discard_rectangles(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_conservative_rasterization(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_conservative_rasterization(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_swapchain_colorspace(&self) -> bool {
        self.ext_swapchain_colorspace && self.supports_khr_surface()
    }
    pub fn enable_ext_swapchain_colorspace(&mut self) {
        self.ext_swapchain_colorspace = true;
        self.enable_khr_surface();
    }
    pub fn supports_ext_hdr_metadata(&self) -> bool {
        self.supports_khr_surface()
    }
    pub fn enable_ext_hdr_metadata(&mut self) {
        self.enable_khr_surface();
    }
    pub fn supports_khr_create_renderpass2(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_create_renderpass2(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_shared_presentable_image(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
            && self.supports_khr_get_surface_capabilities2()
            && self.supports_khr_surface()
    }
    pub fn enable_khr_shared_presentable_image(&mut self) {
        self.enable_khr_get_physical_device_properties2();
        self.enable_khr_get_surface_capabilities2();
        self.enable_khr_surface();
    }
    pub fn supports_khr_external_fence_capabilities(&self) -> bool {
        (self.khr_external_fence_capabilities || self.core_version >= vk::Version::from_raw_parts(1, 1, 0))
            && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_fence_capabilities(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_external_fence_capabilities = true;
        }
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_fence(&self) -> bool {
        self.supports_khr_external_fence_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_fence(&mut self) {
        self.enable_khr_external_fence_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_fence_win32(&self) -> bool {
        self.supports_khr_external_fence_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_fence_win32(&mut self) {
        self.enable_khr_external_fence_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_external_fence_fd(&self) -> bool {
        self.supports_khr_external_fence_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_external_fence_fd(&mut self) {
        self.enable_khr_external_fence_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_performance_query(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_performance_query(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_get_surface_capabilities2(&self) -> bool {
        self.khr_get_surface_capabilities2 && self.supports_khr_surface()
    }
    pub fn enable_khr_get_surface_capabilities2(&mut self) {
        self.khr_get_surface_capabilities2 = true;
        self.enable_khr_surface();
    }
    pub fn supports_khr_variable_pointers(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_variable_pointers(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_get_display_properties2(&self) -> bool {
        self.khr_get_display_properties2 && self.supports_khr_display() && self.supports_khr_surface()
    }
    pub fn enable_khr_get_display_properties2(&mut self) {
        self.khr_get_display_properties2 = true;
        self.enable_khr_display();
        self.enable_khr_surface();
    }
    pub fn supports_mvk_ios_surface(&self) -> bool {
        self.mvk_ios_surface && self.supports_khr_surface()
    }
    pub fn enable_mvk_ios_surface(&mut self) {
        self.mvk_ios_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_mvk_macos_surface(&self) -> bool {
        self.mvk_macos_surface && self.supports_khr_surface()
    }
    pub fn enable_mvk_macos_surface(&mut self) {
        self.mvk_macos_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_ext_external_memory_dma_buf(&self) -> bool {
        self.supports_khr_external_memory_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_external_memory_dma_buf(&mut self) {
        self.enable_khr_external_memory_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_queue_family_foreign(&self) -> bool {
        self.supports_khr_external_memory_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_queue_family_foreign(&mut self) {
        self.enable_khr_external_memory_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_debug_utils(&self) -> bool {
        self.ext_debug_utils
    }
    pub fn enable_ext_debug_utils(&mut self) {
        self.ext_debug_utils = true;
    }
    pub fn supports_android_external_memory_android_hardware_buffer(&self) -> bool {
        self.supports_khr_get_physical_device_properties2() && self.supports_khr_external_memory_capabilities()
    }
    pub fn enable_android_external_memory_android_hardware_buffer(&mut self) {
        self.enable_khr_get_physical_device_properties2();
        self.enable_khr_external_memory_capabilities();
    }
    pub fn supports_ext_sampler_filter_minmax(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_sampler_filter_minmax(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_inline_uniform_block(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_inline_uniform_block(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_sample_locations(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_sample_locations(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_acceleration_structure(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0) && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_acceleration_structure(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_ray_tracing_pipeline(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0) && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_ray_tracing_pipeline(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_ray_query(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0) && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_ray_query(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_sampler_ycbcr_conversion(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_sampler_ycbcr_conversion(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_image_drm_format_modifier(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_image_drm_format_modifier(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_descriptor_indexing(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_descriptor_indexing(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_portability_subset(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_portability_subset(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_shading_rate_image(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_shading_rate_image(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_ray_tracing(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_ray_tracing(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_maintenance3(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_maintenance3(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_8bit_storage(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_8bit_storage(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_external_memory_host(&self) -> bool {
        self.supports_khr_external_memory_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_external_memory_host(&mut self) {
        self.enable_khr_external_memory_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_shader_atomic_int64(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_shader_atomic_int64(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_shader_clock(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_shader_clock(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_amd_shader_core_properties(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_amd_shader_core_properties(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_vertex_attribute_divisor(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_vertex_attribute_divisor(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_driver_properties(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_driver_properties(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_shader_float_controls(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_shader_float_controls(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_depth_stencil_resolve(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_depth_stencil_resolve(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_swapchain_mutable_format(&self) -> bool {
        self.supports_khr_surface()
    }
    pub fn enable_khr_swapchain_mutable_format(&mut self) {
        self.enable_khr_surface();
    }
    pub fn supports_nv_compute_shader_derivatives(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_compute_shader_derivatives(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_mesh_shader(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_mesh_shader(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_fragment_shader_barycentric(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_fragment_shader_barycentric(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_shader_image_footprint(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_shader_image_footprint(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_scissor_exclusive(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_scissor_exclusive(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_device_diagnostic_checkpoints(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_device_diagnostic_checkpoints(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_timeline_semaphore(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_timeline_semaphore(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_intel_shader_integer_functions2(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_intel_shader_integer_functions2(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_pci_bus_info(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_pci_bus_info(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_amd_display_native_hdr(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
            && self.supports_khr_get_surface_capabilities2()
            && self.supports_khr_surface()
    }
    pub fn enable_amd_display_native_hdr(&mut self) {
        self.enable_khr_get_physical_device_properties2();
        self.enable_khr_get_surface_capabilities2();
        self.enable_khr_surface();
    }
    pub fn supports_fuchsia_imagepipe_surface(&self) -> bool {
        self.fuchsia_imagepipe_surface && self.supports_khr_surface()
    }
    pub fn enable_fuchsia_imagepipe_surface(&mut self) {
        self.fuchsia_imagepipe_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_khr_shader_terminate_invocation(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_shader_terminate_invocation(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_metal_surface(&self) -> bool {
        self.ext_metal_surface && self.supports_khr_surface()
    }
    pub fn enable_ext_metal_surface(&mut self) {
        self.ext_metal_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_ext_fragment_density_map(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_fragment_density_map(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_scalar_block_layout(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_scalar_block_layout(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_fragment_shading_rate(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_fragment_shading_rate(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_amd_shader_core_properties2(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_amd_shader_core_properties2(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_shader_image_atomic_int64(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_shader_image_atomic_int64(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_spirv_1_4(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0) && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_spirv_1_4(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_memory_budget(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_memory_budget(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_memory_priority(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_memory_priority(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_surface_protected_capabilities(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
            && self.khr_surface_protected_capabilities
            && self.supports_khr_get_surface_capabilities2()
            && self.supports_khr_surface()
    }
    pub fn enable_khr_surface_protected_capabilities(&mut self) {
        self.khr_surface_protected_capabilities = true;
        self.enable_khr_get_surface_capabilities2();
        self.enable_khr_surface();
    }
    pub fn supports_khr_separate_depth_stencil_layouts(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_separate_depth_stencil_layouts(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_buffer_device_address(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_buffer_device_address(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_validation_features(&self) -> bool {
        self.ext_validation_features
    }
    pub fn enable_ext_validation_features(&mut self) {
        self.ext_validation_features = true;
    }
    pub fn supports_nv_cooperative_matrix(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_cooperative_matrix(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_fragment_shader_interlock(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_fragment_shader_interlock(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_ycbcr_image_arrays(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_ycbcr_image_arrays(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_uniform_buffer_standard_layout(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_uniform_buffer_standard_layout(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_full_screen_exclusive(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
            && self.supports_khr_surface()
            && self.supports_khr_get_surface_capabilities2()
    }
    pub fn enable_ext_full_screen_exclusive(&mut self) {
        self.enable_khr_get_physical_device_properties2();
        self.enable_khr_surface();
        self.enable_khr_get_surface_capabilities2();
    }
    pub fn supports_ext_headless_surface(&self) -> bool {
        self.ext_headless_surface && self.supports_khr_surface()
    }
    pub fn enable_ext_headless_surface(&mut self) {
        self.ext_headless_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_khr_buffer_device_address(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_buffer_device_address(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_line_rasterization(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_line_rasterization(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_shader_atomic_float(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_shader_atomic_float(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_host_query_reset(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_host_query_reset(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_extended_dynamic_state(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_extended_dynamic_state(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_pipeline_executable_properties(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_pipeline_executable_properties(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_shader_demote_to_helper_invocation(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_shader_demote_to_helper_invocation(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_texel_buffer_alignment(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_texel_buffer_alignment(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_qcom_render_pass_transform(&self) -> bool {
        self.supports_khr_surface()
    }
    pub fn enable_qcom_render_pass_transform(&mut self) {
        self.enable_khr_surface();
    }
    pub fn supports_ext_device_memory_report(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_device_memory_report(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_device_diagnostics_config(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_device_diagnostics_config(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_synchronization2(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_synchronization2(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_zero_initialize_workgroup_memory(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_zero_initialize_workgroup_memory(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_fragment_shading_rate_enums(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_nv_fragment_shading_rate_enums(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_ycbcr_2plane_444_formats(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_ycbcr_2plane_444_formats(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_fragment_density_map2(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_fragment_density_map2(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_qcom_rotated_copy_commands(&self) -> bool {
        self.supports_khr_surface()
    }
    pub fn enable_qcom_rotated_copy_commands(&mut self) {
        self.enable_khr_surface();
    }
    pub fn supports_ext_image_robustness(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_image_robustness(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_khr_workgroup_memory_explicit_layout(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_khr_workgroup_memory_explicit_layout(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_4444_formats(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_4444_formats(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_nv_acquire_winrt_display(&self) -> bool {
        self.supports_ext_direct_mode_display() && self.supports_khr_display() && self.supports_khr_surface()
    }
    pub fn enable_nv_acquire_winrt_display(&mut self) {
        self.enable_ext_direct_mode_display();
        self.enable_khr_display();
        self.enable_khr_surface();
    }
    pub fn supports_ext_directfb_surface(&self) -> bool {
        self.ext_directfb_surface && self.supports_khr_surface()
    }
    pub fn enable_ext_directfb_surface(&mut self) {
        self.ext_directfb_surface = true;
        self.enable_khr_surface();
    }
    pub fn supports_valve_mutable_descriptor_type(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_valve_mutable_descriptor_type(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_vertex_input_dynamic_state(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_vertex_input_dynamic_state(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_fuchsia_external_memory(&self) -> bool {
        self.supports_khr_external_memory_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_fuchsia_external_memory(&mut self) {
        self.enable_khr_external_memory_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_fuchsia_external_semaphore(&self) -> bool {
        self.supports_khr_external_semaphore_capabilities() && self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_fuchsia_external_semaphore(&mut self) {
        self.enable_khr_external_semaphore_capabilities();
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn supports_ext_color_write_enable(&self) -> bool {
        self.supports_khr_get_physical_device_properties2()
    }
    pub fn enable_ext_color_write_enable(&mut self) {
        self.enable_khr_get_physical_device_properties2();
    }
    pub fn to_name_vec(&self) -> Vec<&'static CStr> {
        let mut v = Vec::new();
        if self.khr_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface\0") })
        }
        if self.khr_display {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display\0") })
        }
        if self.khr_xlib_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xlib_surface\0") })
        }
        if self.khr_xcb_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xcb_surface\0") })
        }
        if self.khr_wayland_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_wayland_surface\0") })
        }
        if self.khr_android_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_android_surface\0") })
        }
        if self.khr_win32_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_surface\0") })
        }
        if self.ext_debug_report {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_report\0") })
        }
        if self.nv_external_memory_capabilities {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_capabilities\0") })
        }
        if self.khr_get_physical_device_properties2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_physical_device_properties2\0") })
        }
        if self.ext_validation_flags {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_flags\0") })
        }
        if self.nn_vi_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NN_vi_surface\0") })
        }
        if self.khr_device_group_creation {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_device_group_creation\0") })
        }
        if self.khr_external_memory_capabilities {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_capabilities\0") })
        }
        if self.khr_external_semaphore_capabilities {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_capabilities\0") })
        }
        if self.ext_direct_mode_display {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_direct_mode_display\0") })
        }
        if self.ext_acquire_xlib_display {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_xlib_display\0") })
        }
        if self.ext_display_surface_counter {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_surface_counter\0") })
        }
        if self.ext_swapchain_colorspace {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_swapchain_colorspace\0") })
        }
        if self.khr_external_fence_capabilities {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_capabilities\0") })
        }
        if self.khr_get_surface_capabilities2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_surface_capabilities2\0") })
        }
        if self.khr_get_display_properties2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_display_properties2\0") })
        }
        if self.mvk_ios_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_MVK_ios_surface\0") })
        }
        if self.mvk_macos_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_MVK_macos_surface\0") })
        }
        if self.ext_debug_utils {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_utils\0") })
        }
        if self.fuchsia_imagepipe_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_imagepipe_surface\0") })
        }
        if self.ext_metal_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_metal_surface\0") })
        }
        if self.khr_surface_protected_capabilities {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface_protected_capabilities\0") })
        }
        if self.ext_validation_features {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_features\0") })
        }
        if self.ext_headless_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_headless_surface\0") })
        }
        if self.ext_directfb_surface {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_directfb_surface\0") })
        }
        v
    }
}
#[derive(Copy, Clone)]
pub struct Instance {
    pub handle: vk::Instance,
    pub extensions: InstanceExtensions,
    pub fp_destroy_instance: Option<vk::FnDestroyInstance>,
    pub fp_enumerate_physical_devices: Option<vk::FnEnumeratePhysicalDevices>,
    pub fp_get_device_proc_addr: Option<vk::FnGetDeviceProcAddr>,
    pub fp_get_physical_device_properties: Option<vk::FnGetPhysicalDeviceProperties>,
    pub fp_get_physical_device_queue_family_properties: Option<vk::FnGetPhysicalDeviceQueueFamilyProperties>,
    pub fp_get_physical_device_memory_properties: Option<vk::FnGetPhysicalDeviceMemoryProperties>,
    pub fp_get_physical_device_features: Option<vk::FnGetPhysicalDeviceFeatures>,
    pub fp_get_physical_device_format_properties: Option<vk::FnGetPhysicalDeviceFormatProperties>,
    pub fp_get_physical_device_image_format_properties: Option<vk::FnGetPhysicalDeviceImageFormatProperties>,
    pub fp_create_device: Option<vk::FnCreateDevice>,
    pub fp_enumerate_device_layer_properties: Option<vk::FnEnumerateDeviceLayerProperties>,
    pub fp_enumerate_device_extension_properties: Option<vk::FnEnumerateDeviceExtensionProperties>,
    pub fp_get_physical_device_sparse_image_format_properties:
        Option<vk::FnGetPhysicalDeviceSparseImageFormatProperties>,
    pub fp_create_android_surface_khr: Option<vk::FnCreateAndroidSurfaceKHR>,
    pub fp_get_physical_device_display_properties_khr: Option<vk::FnGetPhysicalDeviceDisplayPropertiesKHR>,
    pub fp_get_physical_device_display_plane_properties_khr: Option<vk::FnGetPhysicalDeviceDisplayPlanePropertiesKHR>,
    pub fp_get_display_plane_supported_displays_khr: Option<vk::FnGetDisplayPlaneSupportedDisplaysKHR>,
    pub fp_get_display_mode_properties_khr: Option<vk::FnGetDisplayModePropertiesKHR>,
    pub fp_create_display_mode_khr: Option<vk::FnCreateDisplayModeKHR>,
    pub fp_get_display_plane_capabilities_khr: Option<vk::FnGetDisplayPlaneCapabilitiesKHR>,
    pub fp_create_display_plane_surface_khr: Option<vk::FnCreateDisplayPlaneSurfaceKHR>,
    pub fp_destroy_surface_khr: Option<vk::FnDestroySurfaceKHR>,
    pub fp_get_physical_device_surface_support_khr: Option<vk::FnGetPhysicalDeviceSurfaceSupportKHR>,
    pub fp_get_physical_device_surface_capabilities_khr: Option<vk::FnGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    pub fp_get_physical_device_surface_formats_khr: Option<vk::FnGetPhysicalDeviceSurfaceFormatsKHR>,
    pub fp_get_physical_device_surface_present_modes_khr: Option<vk::FnGetPhysicalDeviceSurfacePresentModesKHR>,
    pub fp_create_vi_surface_nn: Option<vk::FnCreateViSurfaceNN>,
    pub fp_create_wayland_surface_khr: Option<vk::FnCreateWaylandSurfaceKHR>,
    pub fp_get_physical_device_wayland_presentation_support_khr:
        Option<vk::FnGetPhysicalDeviceWaylandPresentationSupportKHR>,
    pub fp_create_win32_surface_khr: Option<vk::FnCreateWin32SurfaceKHR>,
    pub fp_get_physical_device_win32_presentation_support_khr:
        Option<vk::FnGetPhysicalDeviceWin32PresentationSupportKHR>,
    pub fp_create_xlib_surface_khr: Option<vk::FnCreateXlibSurfaceKHR>,
    pub fp_get_physical_device_xlib_presentation_support_khr: Option<vk::FnGetPhysicalDeviceXlibPresentationSupportKHR>,
    pub fp_create_xcb_surface_khr: Option<vk::FnCreateXcbSurfaceKHR>,
    pub fp_get_physical_device_xcb_presentation_support_khr: Option<vk::FnGetPhysicalDeviceXcbPresentationSupportKHR>,
    pub fp_create_direct_fb_surface_ext: Option<vk::FnCreateDirectFBSurfaceEXT>,
    pub fp_get_physical_device_direct_fb_presentation_support_ext:
        Option<vk::FnGetPhysicalDeviceDirectFBPresentationSupportEXT>,
    pub fp_create_image_pipe_surface_fuchsia: Option<vk::FnCreateImagePipeSurfaceFUCHSIA>,
    pub fp_create_debug_report_callback_ext: Option<vk::FnCreateDebugReportCallbackEXT>,
    pub fp_destroy_debug_report_callback_ext: Option<vk::FnDestroyDebugReportCallbackEXT>,
    pub fp_debug_report_message_ext: Option<vk::FnDebugReportMessageEXT>,
    pub fp_get_physical_device_external_image_format_properties_nv:
        Option<vk::FnGetPhysicalDeviceExternalImageFormatPropertiesNV>,
    pub fp_get_physical_device_features2: Option<vk::FnGetPhysicalDeviceFeatures2>,
    pub fp_get_physical_device_properties2: Option<vk::FnGetPhysicalDeviceProperties2>,
    pub fp_get_physical_device_format_properties2: Option<vk::FnGetPhysicalDeviceFormatProperties2>,
    pub fp_get_physical_device_image_format_properties2: Option<vk::FnGetPhysicalDeviceImageFormatProperties2>,
    pub fp_get_physical_device_queue_family_properties2: Option<vk::FnGetPhysicalDeviceQueueFamilyProperties2>,
    pub fp_get_physical_device_memory_properties2: Option<vk::FnGetPhysicalDeviceMemoryProperties2>,
    pub fp_get_physical_device_sparse_image_format_properties2:
        Option<vk::FnGetPhysicalDeviceSparseImageFormatProperties2>,
    pub fp_get_physical_device_external_buffer_properties: Option<vk::FnGetPhysicalDeviceExternalBufferProperties>,
    pub fp_get_physical_device_external_semaphore_properties:
        Option<vk::FnGetPhysicalDeviceExternalSemaphoreProperties>,
    pub fp_get_physical_device_external_fence_properties: Option<vk::FnGetPhysicalDeviceExternalFenceProperties>,
    pub fp_release_display_ext: Option<vk::FnReleaseDisplayEXT>,
    pub fp_acquire_xlib_display_ext: Option<vk::FnAcquireXlibDisplayEXT>,
    pub fp_get_rand_r_output_display_ext: Option<vk::FnGetRandROutputDisplayEXT>,
    pub fp_get_physical_device_surface_capabilities2_ext: Option<vk::FnGetPhysicalDeviceSurfaceCapabilities2EXT>,
    pub fp_enumerate_physical_device_groups: Option<vk::FnEnumeratePhysicalDeviceGroups>,
    pub fp_create_ios_surface_mvk: Option<vk::FnCreateIOSSurfaceMVK>,
    pub fp_create_mac_os_surface_mvk: Option<vk::FnCreateMacOSSurfaceMVK>,
    pub fp_create_metal_surface_ext: Option<vk::FnCreateMetalSurfaceEXT>,
    pub fp_get_physical_device_surface_capabilities2_khr: Option<vk::FnGetPhysicalDeviceSurfaceCapabilities2KHR>,
    pub fp_get_physical_device_surface_formats2_khr: Option<vk::FnGetPhysicalDeviceSurfaceFormats2KHR>,
    pub fp_get_physical_device_display_properties2_khr: Option<vk::FnGetPhysicalDeviceDisplayProperties2KHR>,
    pub fp_get_physical_device_display_plane_properties2_khr: Option<vk::FnGetPhysicalDeviceDisplayPlaneProperties2KHR>,
    pub fp_get_display_mode_properties2_khr: Option<vk::FnGetDisplayModeProperties2KHR>,
    pub fp_get_display_plane_capabilities2_khr: Option<vk::FnGetDisplayPlaneCapabilities2KHR>,
    pub fp_set_debug_utils_object_name_ext: Option<vk::FnSetDebugUtilsObjectNameEXT>,
    pub fp_set_debug_utils_object_tag_ext: Option<vk::FnSetDebugUtilsObjectTagEXT>,
    pub fp_queue_begin_debug_utils_label_ext: Option<vk::FnQueueBeginDebugUtilsLabelEXT>,
    pub fp_queue_end_debug_utils_label_ext: Option<vk::FnQueueEndDebugUtilsLabelEXT>,
    pub fp_queue_insert_debug_utils_label_ext: Option<vk::FnQueueInsertDebugUtilsLabelEXT>,
    pub fp_cmd_begin_debug_utils_label_ext: Option<vk::FnCmdBeginDebugUtilsLabelEXT>,
    pub fp_cmd_end_debug_utils_label_ext: Option<vk::FnCmdEndDebugUtilsLabelEXT>,
    pub fp_cmd_insert_debug_utils_label_ext: Option<vk::FnCmdInsertDebugUtilsLabelEXT>,
    pub fp_create_debug_utils_messenger_ext: Option<vk::FnCreateDebugUtilsMessengerEXT>,
    pub fp_destroy_debug_utils_messenger_ext: Option<vk::FnDestroyDebugUtilsMessengerEXT>,
    pub fp_submit_debug_utils_message_ext: Option<vk::FnSubmitDebugUtilsMessageEXT>,
    pub fp_create_headless_surface_ext: Option<vk::FnCreateHeadlessSurfaceEXT>,
}
impl Instance {
    #[allow(clippy::cognitive_complexity, clippy::nonminimal_bool)]
    pub unsafe fn load(
        loader: &Loader,
        instance: vk::Instance,
        create_info: &vk::InstanceCreateInfo,
    ) -> LoaderResult<Self> {
        let version = create_info
            .p_application_info
            .as_ref()
            .map(|app_info| app_info.api_version)
            .unwrap_or_default();
        let mut extensions = InstanceExtensions::new(version);
        if create_info.enabled_extension_count != 0 {
            for &name_ptr in slice::from_raw_parts(
                create_info.pp_enabled_extension_names,
                create_info.enabled_extension_count as usize,
            ) {
                extensions.enable_by_name(&CStr::from_ptr(name_ptr));
            }
        }
        let f = |name: &CStr| loader.get_instance_proc_addr(Some(instance), name);
        Ok(Self {
            handle: instance,
            extensions,
            fp_destroy_instance: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyInstance\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyInstance".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_enumerate_physical_devices: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkEnumeratePhysicalDevices\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkEnumeratePhysicalDevices".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_device_proc_addr: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceProcAddr\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetDeviceProcAddr".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_physical_device_properties: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceProperties\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetPhysicalDeviceProperties".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_physical_device_queue_family_properties: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceQueueFamilyProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_physical_device_memory_properties: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMemoryProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceMemoryProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_physical_device_features: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFeatures\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetPhysicalDeviceFeatures".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_physical_device_format_properties: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFormatProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceFormatProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_physical_device_image_format_properties: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceImageFormatProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_device: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDevice\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateDevice".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_enumerate_device_layer_properties: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateDeviceLayerProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkEnumerateDeviceLayerProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_enumerate_device_extension_properties: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateDeviceExtensionProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkEnumerateDeviceExtensionProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_physical_device_sparse_image_format_properties: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceSparseImageFormatProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_android_surface_khr: if extensions.khr_android_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateAndroidSurfaceKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_display_properties_khr: if extensions.khr_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_display_plane_properties_khr: if extensions.khr_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_display_plane_supported_displays_khr: if extensions.khr_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneSupportedDisplaysKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_display_mode_properties_khr: if extensions.khr_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetDisplayModePropertiesKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_display_mode_khr: if extensions.khr_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDisplayModeKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_display_plane_capabilities_khr: if extensions.khr_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneCapabilitiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_display_plane_surface_khr: if extensions.khr_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDisplayPlaneSurfaceKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_surface_khr: if extensions.khr_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroySurfaceKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_surface_support_khr: if extensions.khr_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceSupportKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_surface_capabilities_khr: if extensions.khr_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_surface_formats_khr: if extensions.khr_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceFormatsKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_surface_present_modes_khr: if extensions.khr_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfacePresentModesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_vi_surface_nn: if extensions.nn_vi_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateViSurfaceNN\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_wayland_surface_khr: if extensions.khr_wayland_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateWaylandSurfaceKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_wayland_presentation_support_khr: if extensions.khr_wayland_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_win32_surface_khr: if extensions.khr_win32_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateWin32SurfaceKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_win32_presentation_support_khr: if extensions.khr_win32_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_xlib_surface_khr: if extensions.khr_xlib_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateXlibSurfaceKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_xlib_presentation_support_khr: if extensions.khr_xlib_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_xcb_surface_khr: if extensions.khr_xcb_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateXcbSurfaceKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_xcb_presentation_support_khr: if extensions.khr_xcb_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_direct_fb_surface_ext: if extensions.ext_directfb_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDirectFBSurfaceEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_direct_fb_presentation_support_ext: if extensions.ext_directfb_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDirectFBPresentationSupportEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_image_pipe_surface_fuchsia: if extensions.fuchsia_imagepipe_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateImagePipeSurfaceFUCHSIA\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_debug_report_callback_ext: if extensions.ext_debug_report {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDebugReportCallbackEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_debug_report_callback_ext: if extensions.ext_debug_report {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDebugReportCallbackEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_debug_report_message_ext: if extensions.ext_debug_report {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDebugReportMessageEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_external_image_format_properties_nv: if extensions.nv_external_memory_capabilities {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_features2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFeatures2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetPhysicalDeviceFeatures2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_physical_device_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFeatures2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_properties2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceProperties2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetPhysicalDeviceProperties2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_physical_device_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceProperties2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_format_properties2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFormatProperties2\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceFormatProperties2".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_physical_device_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFormatProperties2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_image_format_properties2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties2\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceImageFormatProperties2".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_physical_device_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_queue_family_properties2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties2\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceQueueFamilyProperties2".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_physical_device_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_memory_properties2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMemoryProperties2\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceMemoryProperties2".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_physical_device_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMemoryProperties2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_sparse_image_format_properties2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceSparseImageFormatProperties2".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_physical_device_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_external_buffer_properties: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalBufferProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceExternalBufferProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_external_memory_capabilities {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalBufferPropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_external_semaphore_properties: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalSemaphoreProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceExternalSemaphoreProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_external_semaphore_capabilities {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_external_fence_properties: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalFenceProperties\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetPhysicalDeviceExternalFenceProperties".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_external_fence_capabilities {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalFencePropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_release_display_ext: if extensions.ext_direct_mode_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkReleaseDisplayEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_acquire_xlib_display_ext: if extensions.ext_acquire_xlib_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkAcquireXlibDisplayEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_rand_r_output_display_ext: if extensions.ext_acquire_xlib_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetRandROutputDisplayEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_surface_capabilities2_ext: if extensions.ext_display_surface_counter {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilities2EXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_enumerate_physical_device_groups: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDeviceGroups\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkEnumeratePhysicalDeviceGroups".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_device_group_creation {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDeviceGroupsKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_ios_surface_mvk: if extensions.mvk_ios_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateIOSSurfaceMVK\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_mac_os_surface_mvk: if extensions.mvk_macos_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateMacOSSurfaceMVK\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_metal_surface_ext: if extensions.ext_metal_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateMetalSurfaceEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_surface_capabilities2_khr: if extensions.khr_get_surface_capabilities2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilities2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_surface_formats2_khr: if extensions.khr_get_surface_capabilities2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceFormats2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_display_properties2_khr: if extensions.khr_get_display_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayProperties2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_display_plane_properties2_khr: if extensions.khr_get_display_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_display_mode_properties2_khr: if extensions.khr_get_display_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetDisplayModeProperties2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_display_plane_capabilities2_khr: if extensions.khr_get_display_properties2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneCapabilities2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_set_debug_utils_object_name_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectNameEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_set_debug_utils_object_tag_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectTagEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_queue_begin_debug_utils_label_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkQueueBeginDebugUtilsLabelEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_queue_end_debug_utils_label_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkQueueEndDebugUtilsLabelEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_queue_insert_debug_utils_label_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueInsertDebugUtilsLabelEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_begin_debug_utils_label_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginDebugUtilsLabelEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_end_debug_utils_label_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndDebugUtilsLabelEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_insert_debug_utils_label_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdInsertDebugUtilsLabelEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_debug_utils_messenger_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDebugUtilsMessengerEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_debug_utils_messenger_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDebugUtilsMessengerEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_submit_debug_utils_message_ext: if extensions.ext_debug_utils {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSubmitDebugUtilsMessageEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_headless_surface_ext: if extensions.ext_headless_surface {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateHeadlessSurfaceEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
        })
    }
    pub unsafe fn destroy_instance(&self, p_allocator: Option<&vk::AllocationCallbacks>) {
        let fp = self.fp_destroy_instance.expect("vkDestroyInstance is not loaded");
        (fp)(Some(self.handle), p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn enumerate_physical_devices_to_vec(&self) -> Result<Vec<vk::PhysicalDevice>> {
        let fp = self
            .fp_enumerate_physical_devices
            .expect("vkEnumeratePhysicalDevices is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(self.handle), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_device_proc_addr(&self, device: vk::Device, p_name: &CStr) -> Option<vk::FnVoidFunction> {
        let fp = self.fp_get_device_proc_addr.expect("vkGetDeviceProcAddr is not loaded");
        (fp)(Some(device), p_name.as_ptr())
    }
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        let fp = self
            .fp_get_physical_device_properties
            .expect("vkGetPhysicalDeviceProperties is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn get_physical_device_queue_family_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties> {
        let fp = self
            .fp_get_physical_device_queue_family_properties
            .expect("vkGetPhysicalDeviceQueueFamilyProperties is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        let fp = self
            .fp_get_physical_device_memory_properties
            .expect("vkGetPhysicalDeviceMemoryProperties is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceFeatures {
        let fp = self
            .fp_get_physical_device_features
            .expect("vkGetPhysicalDeviceFeatures is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
    ) -> vk::FormatProperties {
        let fp = self
            .fp_get_physical_device_format_properties
            .expect("vkGetPhysicalDeviceFormatProperties is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), format, res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        ty: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
    ) -> Result<vk::ImageFormatProperties> {
        let fp = self
            .fp_get_physical_device_image_format_properties
            .expect("vkGetPhysicalDeviceImageFormatProperties is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(physical_device),
            format,
            ty,
            tiling,
            usage,
            flags,
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        p_create_info: &vk::DeviceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
        version: vk::Version,
    ) -> result::Result<Device, LoaderError> {
        let fp = self.fp_create_device.expect("vkCreateDevice is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(physical_device),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
        .map_err(LoaderError::Vulkan)
        .and_then(|r| Device::load(&self, r, p_create_info, version))
    }
    pub unsafe fn enumerate_device_layer_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::LayerProperties>> {
        let fp = self
            .fp_enumerate_device_layer_properties
            .expect("vkEnumerateDeviceLayerProperties is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn enumerate_device_extension_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_layer_name: Option<&CStr>,
    ) -> Result<Vec<vk::ExtensionProperties>> {
        let fp = self
            .fp_enumerate_device_extension_properties
            .expect("vkEnumerateDeviceExtensionProperties is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(
            Some(physical_device),
            p_layer_name.map_or(ptr::null(), |s| s.as_ptr()),
            len.as_mut_ptr(),
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(
            Some(physical_device),
            p_layer_name.map_or(ptr::null(), |s| s.as_ptr()),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        ty: vk::ImageType,
        samples: vk::SampleCountFlags,
        usage: vk::ImageUsageFlags,
        tiling: vk::ImageTiling,
    ) -> Vec<vk::SparseImageFormatProperties> {
        let fp = self
            .fp_get_physical_device_sparse_image_format_properties
            .expect("vkGetPhysicalDeviceSparseImageFormatProperties is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(
            Some(physical_device),
            format,
            ty,
            samples,
            usage,
            tiling,
            len.as_mut_ptr(),
            ptr::null_mut(),
        );
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(
            Some(physical_device),
            format,
            ty,
            samples,
            usage,
            tiling,
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        v
    }
    pub unsafe fn create_android_surface_khr(
        &self,
        p_create_info: &vk::AndroidSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_android_surface_khr
            .expect("vkCreateAndroidSurfaceKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_display_properties_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPropertiesKHR>> {
        let fp = self
            .fp_get_physical_device_display_properties_khr
            .expect("vkGetPhysicalDeviceDisplayPropertiesKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_physical_device_display_plane_properties_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPlanePropertiesKHR>> {
        let fp = self
            .fp_get_physical_device_display_plane_properties_khr
            .expect("vkGetPhysicalDeviceDisplayPlanePropertiesKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_display_plane_supported_displays_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        plane_index: u32,
    ) -> Result<Vec<vk::DisplayKHR>> {
        let fp = self
            .fp_get_display_plane_supported_displays_khr
            .expect("vkGetDisplayPlaneSupportedDisplaysKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), plane_index, len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), plane_index, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_display_mode_properties_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> Result<Vec<vk::DisplayModePropertiesKHR>> {
        let fp = self
            .fp_get_display_mode_properties_khr
            .expect("vkGetDisplayModePropertiesKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), Some(display), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), Some(display), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        p_create_info: &vk::DisplayModeCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DisplayModeKHR> {
        let fp = self
            .fp_create_display_mode_khr
            .expect("vkCreateDisplayModeKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(physical_device),
            Some(display),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        mode: vk::DisplayModeKHR,
        plane_index: u32,
    ) -> Result<vk::DisplayPlaneCapabilitiesKHR> {
        let fp = self
            .fp_get_display_plane_capabilities_khr
            .expect("vkGetDisplayPlaneCapabilitiesKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(physical_device), Some(mode), plane_index, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        p_create_info: &vk::DisplaySurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_display_plane_surface_khr
            .expect("vkCreateDisplayPlaneSurfaceKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_surface_khr(
        &self,
        surface: Option<vk::SurfaceKHR>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self.fp_destroy_surface_khr.expect("vkDestroySurfaceKHR is not loaded");
        (fp)(Some(self.handle), surface, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        surface: vk::SurfaceKHR,
    ) -> Result<bool> {
        let fp = self
            .fp_get_physical_device_surface_support_khr
            .expect("vkGetPhysicalDeviceSurfaceSupportKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(physical_device),
            queue_family_index,
            Some(surface),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
        .map(|r| r != vk::FALSE)
    }
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<vk::SurfaceCapabilitiesKHR> {
        let fp = self
            .fp_get_physical_device_surface_capabilities_khr
            .expect("vkGetPhysicalDeviceSurfaceCapabilitiesKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(physical_device), Some(surface), res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_surface_formats_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Vec<vk::SurfaceFormatKHR>> {
        let fp = self
            .fp_get_physical_device_surface_formats_khr
            .expect("vkGetPhysicalDeviceSurfaceFormatsKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), Some(surface), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), Some(surface), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_physical_device_surface_present_modes_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Vec<vk::PresentModeKHR>> {
        let fp = self
            .fp_get_physical_device_surface_present_modes_khr
            .expect("vkGetPhysicalDeviceSurfacePresentModesKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), Some(surface), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), Some(surface), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_vi_surface_nn(
        &self,
        p_create_info: &vk::ViSurfaceCreateInfoNN,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self.fp_create_vi_surface_nn.expect("vkCreateViSurfaceNN is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_wayland_surface_khr(
        &self,
        p_create_info: &vk::WaylandSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_wayland_surface_khr
            .expect("vkCreateWaylandSurfaceKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        display: &mut vk::wl_display,
    ) -> vk::Bool32 {
        let fp = self
            .fp_get_physical_device_wayland_presentation_support_khr
            .expect("vkGetPhysicalDeviceWaylandPresentationSupportKHR is not loaded");
        (fp)(Some(physical_device), queue_family_index, display)
    }
    pub unsafe fn create_win32_surface_khr(
        &self,
        p_create_info: &vk::Win32SurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_win32_surface_khr
            .expect("vkCreateWin32SurfaceKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> vk::Bool32 {
        let fp = self
            .fp_get_physical_device_win32_presentation_support_khr
            .expect("vkGetPhysicalDeviceWin32PresentationSupportKHR is not loaded");
        (fp)(Some(physical_device), queue_family_index)
    }
    pub unsafe fn create_xlib_surface_khr(
        &self,
        p_create_info: &vk::XlibSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_xlib_surface_khr
            .expect("vkCreateXlibSurfaceKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        dpy: &mut vk::Display,
        visual_id: vk::VisualID,
    ) -> vk::Bool32 {
        let fp = self
            .fp_get_physical_device_xlib_presentation_support_khr
            .expect("vkGetPhysicalDeviceXlibPresentationSupportKHR is not loaded");
        (fp)(Some(physical_device), queue_family_index, dpy, visual_id)
    }
    pub unsafe fn create_xcb_surface_khr(
        &self,
        p_create_info: &vk::XcbSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_xcb_surface_khr
            .expect("vkCreateXcbSurfaceKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        connection: &mut vk::xcb_connection_t,
        visual_id: vk::xcb_visualid_t,
    ) -> vk::Bool32 {
        let fp = self
            .fp_get_physical_device_xcb_presentation_support_khr
            .expect("vkGetPhysicalDeviceXcbPresentationSupportKHR is not loaded");
        (fp)(Some(physical_device), queue_family_index, connection, visual_id)
    }
    pub unsafe fn create_direct_fb_surface_ext(
        &self,
        p_create_info: &vk::DirectFBSurfaceCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_direct_fb_surface_ext
            .expect("vkCreateDirectFBSurfaceEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_direct_fb_presentation_support_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        dfb: &mut vk::IDirectFB,
    ) -> vk::Bool32 {
        let fp = self
            .fp_get_physical_device_direct_fb_presentation_support_ext
            .expect("vkGetPhysicalDeviceDirectFBPresentationSupportEXT is not loaded");
        (fp)(Some(physical_device), queue_family_index, dfb)
    }
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        p_create_info: &vk::ImagePipeSurfaceCreateInfoFUCHSIA,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_image_pipe_surface_fuchsia
            .expect("vkCreateImagePipeSurfaceFUCHSIA is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        p_create_info: &vk::DebugReportCallbackCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DebugReportCallbackEXT> {
        let fp = self
            .fp_create_debug_report_callback_ext
            .expect("vkCreateDebugReportCallbackEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        callback: Option<vk::DebugReportCallbackEXT>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_debug_report_callback_ext
            .expect("vkDestroyDebugReportCallbackEXT is not loaded");
        (fp)(Some(self.handle), callback, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn debug_report_message_ext(
        &self,
        flags: vk::DebugReportFlagsEXT,
        object_type: vk::DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: &CStr,
        p_message: &CStr,
    ) {
        let fp = self
            .fp_debug_report_message_ext
            .expect("vkDebugReportMessageEXT is not loaded");
        (fp)(
            Some(self.handle),
            flags,
            object_type,
            object,
            location,
            message_code,
            p_layer_prefix.as_ptr(),
            p_message.as_ptr(),
        );
    }
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        ty: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
        external_handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<vk::ExternalImageFormatPropertiesNV> {
        let fp = self
            .fp_get_physical_device_external_image_format_properties_nv
            .expect("vkGetPhysicalDeviceExternalImageFormatPropertiesNV is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(physical_device),
            format,
            ty,
            tiling,
            usage,
            flags,
            external_handle_type,
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_features: &mut vk::PhysicalDeviceFeatures2,
    ) {
        let fp = self
            .fp_get_physical_device_features2
            .expect("vkGetPhysicalDeviceFeatures2 is not loaded");
        (fp)(Some(physical_device), p_features);
    }
    pub unsafe fn get_physical_device_features2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_features: &mut vk::PhysicalDeviceFeatures2,
    ) {
        let fp = self
            .fp_get_physical_device_features2
            .expect("vkGetPhysicalDeviceFeatures2KHR is not loaded");
        (fp)(Some(physical_device), p_features);
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_properties: &mut vk::PhysicalDeviceProperties2,
    ) {
        let fp = self
            .fp_get_physical_device_properties2
            .expect("vkGetPhysicalDeviceProperties2 is not loaded");
        (fp)(Some(physical_device), p_properties);
    }
    pub unsafe fn get_physical_device_properties2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_properties: &mut vk::PhysicalDeviceProperties2,
    ) {
        let fp = self
            .fp_get_physical_device_properties2
            .expect("vkGetPhysicalDeviceProperties2KHR is not loaded");
        (fp)(Some(physical_device), p_properties);
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        p_format_properties: &mut vk::FormatProperties2,
    ) {
        let fp = self
            .fp_get_physical_device_format_properties2
            .expect("vkGetPhysicalDeviceFormatProperties2 is not loaded");
        (fp)(Some(physical_device), format, p_format_properties);
    }
    pub unsafe fn get_physical_device_format_properties2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        p_format_properties: &mut vk::FormatProperties2,
    ) {
        let fp = self
            .fp_get_physical_device_format_properties2
            .expect("vkGetPhysicalDeviceFormatProperties2KHR is not loaded");
        (fp)(Some(physical_device), format, p_format_properties);
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_image_format_info: &vk::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: &mut vk::ImageFormatProperties2,
    ) -> Result<()> {
        let fp = self
            .fp_get_physical_device_image_format_properties2
            .expect("vkGetPhysicalDeviceImageFormatProperties2 is not loaded");
        let err = (fp)(Some(physical_device), p_image_format_info, p_image_format_properties);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_image_format_properties2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_image_format_info: &vk::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: &mut vk::ImageFormatProperties2,
    ) -> Result<()> {
        let fp = self
            .fp_get_physical_device_image_format_properties2
            .expect("vkGetPhysicalDeviceImageFormatProperties2KHR is not loaded");
        let err = (fp)(Some(physical_device), p_image_format_info, p_image_format_properties);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_queue_family_properties2_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties2> {
        let fp = self
            .fp_get_physical_device_queue_family_properties2
            .expect("vkGetPhysicalDeviceQueueFamilyProperties2 is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn get_physical_device_queue_family_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties2> {
        let fp = self
            .fp_get_physical_device_queue_family_properties2
            .expect("vkGetPhysicalDeviceQueueFamilyProperties2KHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_memory_properties: &mut vk::PhysicalDeviceMemoryProperties2,
    ) {
        let fp = self
            .fp_get_physical_device_memory_properties2
            .expect("vkGetPhysicalDeviceMemoryProperties2 is not loaded");
        (fp)(Some(physical_device), p_memory_properties);
    }
    pub unsafe fn get_physical_device_memory_properties2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_memory_properties: &mut vk::PhysicalDeviceMemoryProperties2,
    ) {
        let fp = self
            .fp_get_physical_device_memory_properties2
            .expect("vkGetPhysicalDeviceMemoryProperties2KHR is not loaded");
        (fp)(Some(physical_device), p_memory_properties);
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> Vec<vk::SparseImageFormatProperties2> {
        let fp = self
            .fp_get_physical_device_sparse_image_format_properties2
            .expect("vkGetPhysicalDeviceSparseImageFormatProperties2 is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), p_format_info, len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(physical_device), p_format_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> Vec<vk::SparseImageFormatProperties2> {
        let fp = self
            .fp_get_physical_device_sparse_image_format_properties2
            .expect("vkGetPhysicalDeviceSparseImageFormatProperties2KHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), p_format_info, len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(physical_device), p_format_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: &mut vk::ExternalBufferProperties,
    ) {
        let fp = self
            .fp_get_physical_device_external_buffer_properties
            .expect("vkGetPhysicalDeviceExternalBufferProperties is not loaded");
        (fp)(
            Some(physical_device),
            p_external_buffer_info,
            p_external_buffer_properties,
        );
    }
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: &mut vk::ExternalBufferProperties,
    ) {
        let fp = self
            .fp_get_physical_device_external_buffer_properties
            .expect("vkGetPhysicalDeviceExternalBufferPropertiesKHR is not loaded");
        (fp)(
            Some(physical_device),
            p_external_buffer_info,
            p_external_buffer_properties,
        );
    }
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: &mut vk::ExternalSemaphoreProperties,
    ) {
        let fp = self
            .fp_get_physical_device_external_semaphore_properties
            .expect("vkGetPhysicalDeviceExternalSemaphoreProperties is not loaded");
        (fp)(
            Some(physical_device),
            p_external_semaphore_info,
            p_external_semaphore_properties,
        );
    }
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: &mut vk::ExternalSemaphoreProperties,
    ) {
        let fp = self
            .fp_get_physical_device_external_semaphore_properties
            .expect("vkGetPhysicalDeviceExternalSemaphorePropertiesKHR is not loaded");
        (fp)(
            Some(physical_device),
            p_external_semaphore_info,
            p_external_semaphore_properties,
        );
    }
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: &mut vk::ExternalFenceProperties,
    ) {
        let fp = self
            .fp_get_physical_device_external_fence_properties
            .expect("vkGetPhysicalDeviceExternalFenceProperties is not loaded");
        (fp)(
            Some(physical_device),
            p_external_fence_info,
            p_external_fence_properties,
        );
    }
    pub unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: &mut vk::ExternalFenceProperties,
    ) {
        let fp = self
            .fp_get_physical_device_external_fence_properties
            .expect("vkGetPhysicalDeviceExternalFencePropertiesKHR is not loaded");
        (fp)(
            Some(physical_device),
            p_external_fence_info,
            p_external_fence_properties,
        );
    }
    pub unsafe fn release_display_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> Result<()> {
        let fp = self.fp_release_display_ext.expect("vkReleaseDisplayEXT is not loaded");
        let err = (fp)(Some(physical_device), Some(display));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        dpy: &mut vk::Display,
        display: vk::DisplayKHR,
    ) -> Result<()> {
        let fp = self
            .fp_acquire_xlib_display_ext
            .expect("vkAcquireXlibDisplayEXT is not loaded");
        let err = (fp)(Some(physical_device), dpy, Some(display));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        dpy: &mut vk::Display,
        rr_output: vk::RROutput,
    ) -> Result<vk::DisplayKHR> {
        let fp = self
            .fp_get_rand_r_output_display_ext
            .expect("vkGetRandROutputDisplayEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(physical_device), dpy, rr_output, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_surface_capabilities: &mut vk::SurfaceCapabilities2EXT,
    ) -> Result<()> {
        let fp = self
            .fp_get_physical_device_surface_capabilities2_ext
            .expect("vkGetPhysicalDeviceSurfaceCapabilities2EXT is not loaded");
        let err = (fp)(Some(physical_device), Some(surface), p_surface_capabilities);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn enumerate_physical_device_groups_to_vec(&self) -> Result<Vec<vk::PhysicalDeviceGroupProperties>> {
        let fp = self
            .fp_enumerate_physical_device_groups
            .expect("vkEnumeratePhysicalDeviceGroups is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(self.handle), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn enumerate_physical_device_groups_khr_to_vec(&self) -> Result<Vec<vk::PhysicalDeviceGroupProperties>> {
        let fp = self
            .fp_enumerate_physical_device_groups
            .expect("vkEnumeratePhysicalDeviceGroupsKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(self.handle), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_ios_surface_mvk(
        &self,
        p_create_info: &vk::IOSSurfaceCreateInfoMVK,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_ios_surface_mvk
            .expect("vkCreateIOSSurfaceMVK is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_mac_os_surface_mvk(
        &self,
        p_create_info: &vk::MacOSSurfaceCreateInfoMVK,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_mac_os_surface_mvk
            .expect("vkCreateMacOSSurfaceMVK is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_metal_surface_ext(
        &self,
        p_create_info: &vk::MetalSurfaceCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_metal_surface_ext
            .expect("vkCreateMetalSurfaceEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
        p_surface_capabilities: &mut vk::SurfaceCapabilities2KHR,
    ) -> Result<()> {
        let fp = self
            .fp_get_physical_device_surface_capabilities2_khr
            .expect("vkGetPhysicalDeviceSurfaceCapabilities2KHR is not loaded");
        let err = (fp)(Some(physical_device), p_surface_info, p_surface_capabilities);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_surface_formats2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<Vec<vk::SurfaceFormat2KHR>> {
        let fp = self
            .fp_get_physical_device_surface_formats2_khr
            .expect("vkGetPhysicalDeviceSurfaceFormats2KHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), p_surface_info, len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), p_surface_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_physical_device_display_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayProperties2KHR>> {
        let fp = self
            .fp_get_physical_device_display_properties2_khr
            .expect("vkGetPhysicalDeviceDisplayProperties2KHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_physical_device_display_plane_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPlaneProperties2KHR>> {
        let fp = self
            .fp_get_physical_device_display_plane_properties2_khr
            .expect("vkGetPhysicalDeviceDisplayPlaneProperties2KHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_display_mode_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> Result<Vec<vk::DisplayModeProperties2KHR>> {
        let fp = self
            .fp_get_display_mode_properties2_khr
            .expect("vkGetDisplayModeProperties2KHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), Some(display), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), Some(display), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_display_plane_info: &vk::DisplayPlaneInfo2KHR,
        p_capabilities: &mut vk::DisplayPlaneCapabilities2KHR,
    ) -> Result<()> {
        let fp = self
            .fp_get_display_plane_capabilities2_khr
            .expect("vkGetDisplayPlaneCapabilities2KHR is not loaded");
        let err = (fp)(Some(physical_device), p_display_plane_info, p_capabilities);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: vk::Device,
        p_name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) -> Result<()> {
        let fp = self
            .fp_set_debug_utils_object_name_ext
            .expect("vkSetDebugUtilsObjectNameEXT is not loaded");
        let err = (fp)(Some(device), p_name_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        device: vk::Device,
        p_tag_info: &vk::DebugUtilsObjectTagInfoEXT,
    ) -> Result<()> {
        let fp = self
            .fp_set_debug_utils_object_tag_ext
            .expect("vkSetDebugUtilsObjectTagEXT is not loaded");
        let err = (fp)(Some(device), p_tag_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn queue_begin_debug_utils_label_ext(&self, queue: vk::Queue, p_label_info: &vk::DebugUtilsLabelEXT) {
        let fp = self
            .fp_queue_begin_debug_utils_label_ext
            .expect("vkQueueBeginDebugUtilsLabelEXT is not loaded");
        (fp)(Some(queue), p_label_info);
    }
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: vk::Queue) {
        let fp = self
            .fp_queue_end_debug_utils_label_ext
            .expect("vkQueueEndDebugUtilsLabelEXT is not loaded");
        (fp)(Some(queue));
    }
    pub unsafe fn queue_insert_debug_utils_label_ext(&self, queue: vk::Queue, p_label_info: &vk::DebugUtilsLabelEXT) {
        let fp = self
            .fp_queue_insert_debug_utils_label_ext
            .expect("vkQueueInsertDebugUtilsLabelEXT is not loaded");
        (fp)(Some(queue), p_label_info);
    }
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_label_info: &vk::DebugUtilsLabelEXT,
    ) {
        let fp = self
            .fp_cmd_begin_debug_utils_label_ext
            .expect("vkCmdBeginDebugUtilsLabelEXT is not loaded");
        (fp)(Some(command_buffer), p_label_info);
    }
    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: vk::CommandBuffer) {
        let fp = self
            .fp_cmd_end_debug_utils_label_ext
            .expect("vkCmdEndDebugUtilsLabelEXT is not loaded");
        (fp)(Some(command_buffer));
    }
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_label_info: &vk::DebugUtilsLabelEXT,
    ) {
        let fp = self
            .fp_cmd_insert_debug_utils_label_ext
            .expect("vkCmdInsertDebugUtilsLabelEXT is not loaded");
        (fp)(Some(command_buffer), p_label_info);
    }
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        p_create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DebugUtilsMessengerEXT> {
        let fp = self
            .fp_create_debug_utils_messenger_ext
            .expect("vkCreateDebugUtilsMessengerEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        messenger: Option<vk::DebugUtilsMessengerEXT>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_debug_utils_messenger_ext
            .expect("vkDestroyDebugUtilsMessengerEXT is not loaded");
        (fp)(Some(self.handle), messenger, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: &vk::DebugUtilsMessengerCallbackDataEXT,
    ) {
        let fp = self
            .fp_submit_debug_utils_message_ext
            .expect("vkSubmitDebugUtilsMessageEXT is not loaded");
        (fp)(Some(self.handle), message_severity, message_types, p_callback_data);
    }
    pub unsafe fn create_headless_surface_ext(
        &self,
        p_create_info: &vk::HeadlessSurfaceCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let fp = self
            .fp_create_headless_surface_ext
            .expect("vkCreateHeadlessSurfaceEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct DeviceExtensions {
    pub core_version: vk::Version,
    pub khr_swapchain: bool,
    pub khr_display_swapchain: bool,
    pub nv_glsl_shader: bool,
    pub ext_depth_range_unrestricted: bool,
    pub khr_sampler_mirror_clamp_to_edge: bool,
    pub img_filter_cubic: bool,
    pub amd_rasterization_order: bool,
    pub amd_shader_trinary_minmax: bool,
    pub amd_shader_explicit_vertex_parameter: bool,
    pub ext_debug_marker: bool,
    pub amd_gcn_shader: bool,
    pub nv_dedicated_allocation: bool,
    pub ext_transform_feedback: bool,
    pub nvx_image_view_handle: bool,
    pub amd_draw_indirect_count: bool,
    pub amd_negative_viewport_height: bool,
    pub amd_gpu_shader_half_float: bool,
    pub amd_shader_ballot: bool,
    pub amd_texture_gather_bias_lod: bool,
    pub amd_shader_info: bool,
    pub amd_shader_image_load_store_lod: bool,
    pub nv_corner_sampled_image: bool,
    pub khr_multiview: bool,
    pub img_format_pvrtc: bool,
    pub nv_external_memory: bool,
    pub nv_external_memory_win32: bool,
    pub nv_win32_keyed_mutex: bool,
    pub khr_device_group: bool,
    pub khr_shader_draw_parameters: bool,
    pub ext_shader_subgroup_ballot: bool,
    pub ext_shader_subgroup_vote: bool,
    pub ext_texture_compression_astc_hdr: bool,
    pub ext_astc_decode_mode: bool,
    pub khr_maintenance1: bool,
    pub khr_external_memory: bool,
    pub khr_external_memory_win32: bool,
    pub khr_external_memory_fd: bool,
    pub khr_win32_keyed_mutex: bool,
    pub khr_external_semaphore: bool,
    pub khr_external_semaphore_win32: bool,
    pub khr_external_semaphore_fd: bool,
    pub khr_push_descriptor: bool,
    pub ext_conditional_rendering: bool,
    pub khr_shader_float16_int8: bool,
    pub khr_16bit_storage: bool,
    pub khr_incremental_present: bool,
    pub khr_descriptor_update_template: bool,
    pub nv_clip_space_w_scaling: bool,
    pub ext_display_control: bool,
    pub google_display_timing: bool,
    pub nv_sample_mask_override_coverage: bool,
    pub nv_geometry_shader_passthrough: bool,
    pub nv_viewport_array2: bool,
    pub nvx_multiview_per_view_attributes: bool,
    pub nv_viewport_swizzle: bool,
    pub ext_discard_rectangles: bool,
    pub ext_conservative_rasterization: bool,
    pub ext_depth_clip_enable: bool,
    pub ext_hdr_metadata: bool,
    pub khr_imageless_framebuffer: bool,
    pub khr_create_renderpass2: bool,
    pub khr_shared_presentable_image: bool,
    pub khr_external_fence: bool,
    pub khr_external_fence_win32: bool,
    pub khr_external_fence_fd: bool,
    pub khr_performance_query: bool,
    pub khr_maintenance2: bool,
    pub khr_variable_pointers: bool,
    pub ext_external_memory_dma_buf: bool,
    pub ext_queue_family_foreign: bool,
    pub khr_dedicated_allocation: bool,
    pub android_external_memory_android_hardware_buffer: bool,
    pub ext_sampler_filter_minmax: bool,
    pub khr_storage_buffer_storage_class: bool,
    pub amd_gpu_shader_int16: bool,
    pub amd_mixed_attachment_samples: bool,
    pub amd_shader_fragment_mask: bool,
    pub ext_inline_uniform_block: bool,
    pub ext_shader_stencil_export: bool,
    pub ext_sample_locations: bool,
    pub khr_relaxed_block_layout: bool,
    pub khr_get_memory_requirements2: bool,
    pub khr_image_format_list: bool,
    pub ext_blend_operation_advanced: bool,
    pub nv_fragment_coverage_to_color: bool,
    pub khr_acceleration_structure: bool,
    pub khr_ray_tracing_pipeline: bool,
    pub khr_ray_query: bool,
    pub nv_framebuffer_mixed_samples: bool,
    pub nv_fill_rectangle: bool,
    pub nv_shader_sm_builtins: bool,
    pub ext_post_depth_coverage: bool,
    pub khr_sampler_ycbcr_conversion: bool,
    pub khr_bind_memory2: bool,
    pub ext_image_drm_format_modifier: bool,
    pub ext_validation_cache: bool,
    pub ext_descriptor_indexing: bool,
    pub ext_shader_viewport_index_layer: bool,
    pub khr_portability_subset: bool,
    pub nv_shading_rate_image: bool,
    pub nv_ray_tracing: bool,
    pub nv_representative_fragment_test: bool,
    pub khr_maintenance3: bool,
    pub khr_draw_indirect_count: bool,
    pub ext_filter_cubic: bool,
    pub qcom_render_pass_shader_resolve: bool,
    pub ext_global_priority: bool,
    pub khr_shader_subgroup_extended_types: bool,
    pub khr_8bit_storage: bool,
    pub ext_external_memory_host: bool,
    pub amd_buffer_marker: bool,
    pub khr_shader_atomic_int64: bool,
    pub khr_shader_clock: bool,
    pub amd_pipeline_compiler_control: bool,
    pub ext_calibrated_timestamps: bool,
    pub amd_shader_core_properties: bool,
    pub amd_memory_overallocation_behavior: bool,
    pub ext_vertex_attribute_divisor: bool,
    pub ext_pipeline_creation_feedback: bool,
    pub khr_driver_properties: bool,
    pub khr_shader_float_controls: bool,
    pub nv_shader_subgroup_partitioned: bool,
    pub khr_depth_stencil_resolve: bool,
    pub khr_swapchain_mutable_format: bool,
    pub nv_compute_shader_derivatives: bool,
    pub nv_mesh_shader: bool,
    pub nv_fragment_shader_barycentric: bool,
    pub nv_shader_image_footprint: bool,
    pub nv_scissor_exclusive: bool,
    pub nv_device_diagnostic_checkpoints: bool,
    pub khr_timeline_semaphore: bool,
    pub intel_shader_integer_functions2: bool,
    pub intel_performance_query: bool,
    pub khr_vulkan_memory_model: bool,
    pub ext_pci_bus_info: bool,
    pub amd_display_native_hdr: bool,
    pub khr_shader_terminate_invocation: bool,
    pub ext_fragment_density_map: bool,
    pub ext_scalar_block_layout: bool,
    pub google_hlsl_functionality1: bool,
    pub google_decorate_string: bool,
    pub ext_subgroup_size_control: bool,
    pub khr_fragment_shading_rate: bool,
    pub amd_shader_core_properties2: bool,
    pub amd_device_coherent_memory: bool,
    pub ext_shader_image_atomic_int64: bool,
    pub khr_spirv_1_4: bool,
    pub ext_memory_budget: bool,
    pub ext_memory_priority: bool,
    pub nv_dedicated_allocation_image_aliasing: bool,
    pub khr_separate_depth_stencil_layouts: bool,
    pub ext_buffer_device_address: bool,
    pub ext_tooling_info: bool,
    pub ext_separate_stencil_usage: bool,
    pub nv_cooperative_matrix: bool,
    pub nv_coverage_reduction_mode: bool,
    pub ext_fragment_shader_interlock: bool,
    pub ext_ycbcr_image_arrays: bool,
    pub khr_uniform_buffer_standard_layout: bool,
    pub ext_full_screen_exclusive: bool,
    pub khr_buffer_device_address: bool,
    pub ext_line_rasterization: bool,
    pub ext_shader_atomic_float: bool,
    pub ext_host_query_reset: bool,
    pub ext_index_type_uint8: bool,
    pub ext_extended_dynamic_state: bool,
    pub khr_deferred_host_operations: bool,
    pub khr_pipeline_executable_properties: bool,
    pub ext_shader_demote_to_helper_invocation: bool,
    pub nv_device_generated_commands: bool,
    pub nv_inherited_viewport_scissor: bool,
    pub ext_texel_buffer_alignment: bool,
    pub qcom_render_pass_transform: bool,
    pub ext_device_memory_report: bool,
    pub ext_robustness2: bool,
    pub ext_custom_border_color: bool,
    pub google_user_type: bool,
    pub khr_pipeline_library: bool,
    pub khr_shader_non_semantic_info: bool,
    pub ext_private_data: bool,
    pub ext_pipeline_creation_cache_control: bool,
    pub nv_device_diagnostics_config: bool,
    pub qcom_render_pass_store_ops: bool,
    pub khr_synchronization2: bool,
    pub khr_zero_initialize_workgroup_memory: bool,
    pub nv_fragment_shading_rate_enums: bool,
    pub ext_ycbcr_2plane_444_formats: bool,
    pub ext_fragment_density_map2: bool,
    pub qcom_rotated_copy_commands: bool,
    pub ext_image_robustness: bool,
    pub khr_workgroup_memory_explicit_layout: bool,
    pub khr_copy_commands2: bool,
    pub ext_4444_formats: bool,
    pub nv_acquire_winrt_display: bool,
    pub valve_mutable_descriptor_type: bool,
    pub ext_vertex_input_dynamic_state: bool,
    pub fuchsia_external_memory: bool,
    pub fuchsia_external_semaphore: bool,
    pub ext_color_write_enable: bool,
}
impl DeviceExtensions {
    fn enable_by_name(&mut self, name: &CStr) {
        match name.to_bytes() {
            b"VK_KHR_swapchain" => self.khr_swapchain = true,
            b"VK_KHR_display_swapchain" => self.khr_display_swapchain = true,
            b"VK_NV_glsl_shader" => self.nv_glsl_shader = true,
            b"VK_EXT_depth_range_unrestricted" => self.ext_depth_range_unrestricted = true,
            b"VK_KHR_sampler_mirror_clamp_to_edge" => self.khr_sampler_mirror_clamp_to_edge = true,
            b"VK_IMG_filter_cubic" => self.img_filter_cubic = true,
            b"VK_AMD_rasterization_order" => self.amd_rasterization_order = true,
            b"VK_AMD_shader_trinary_minmax" => self.amd_shader_trinary_minmax = true,
            b"VK_AMD_shader_explicit_vertex_parameter" => self.amd_shader_explicit_vertex_parameter = true,
            b"VK_EXT_debug_marker" => self.ext_debug_marker = true,
            b"VK_AMD_gcn_shader" => self.amd_gcn_shader = true,
            b"VK_NV_dedicated_allocation" => self.nv_dedicated_allocation = true,
            b"VK_EXT_transform_feedback" => self.ext_transform_feedback = true,
            b"VK_NVX_image_view_handle" => self.nvx_image_view_handle = true,
            b"VK_AMD_draw_indirect_count" => self.amd_draw_indirect_count = true,
            b"VK_AMD_negative_viewport_height" => self.amd_negative_viewport_height = true,
            b"VK_AMD_gpu_shader_half_float" => self.amd_gpu_shader_half_float = true,
            b"VK_AMD_shader_ballot" => self.amd_shader_ballot = true,
            b"VK_AMD_texture_gather_bias_lod" => self.amd_texture_gather_bias_lod = true,
            b"VK_AMD_shader_info" => self.amd_shader_info = true,
            b"VK_AMD_shader_image_load_store_lod" => self.amd_shader_image_load_store_lod = true,
            b"VK_NV_corner_sampled_image" => self.nv_corner_sampled_image = true,
            b"VK_KHR_multiview" => self.khr_multiview = true,
            b"VK_IMG_format_pvrtc" => self.img_format_pvrtc = true,
            b"VK_NV_external_memory" => self.nv_external_memory = true,
            b"VK_NV_external_memory_win32" => self.nv_external_memory_win32 = true,
            b"VK_NV_win32_keyed_mutex" => self.nv_win32_keyed_mutex = true,
            b"VK_KHR_device_group" => self.khr_device_group = true,
            b"VK_KHR_shader_draw_parameters" => self.khr_shader_draw_parameters = true,
            b"VK_EXT_shader_subgroup_ballot" => self.ext_shader_subgroup_ballot = true,
            b"VK_EXT_shader_subgroup_vote" => self.ext_shader_subgroup_vote = true,
            b"VK_EXT_texture_compression_astc_hdr" => self.ext_texture_compression_astc_hdr = true,
            b"VK_EXT_astc_decode_mode" => self.ext_astc_decode_mode = true,
            b"VK_KHR_maintenance1" => self.khr_maintenance1 = true,
            b"VK_KHR_external_memory" => self.khr_external_memory = true,
            b"VK_KHR_external_memory_win32" => self.khr_external_memory_win32 = true,
            b"VK_KHR_external_memory_fd" => self.khr_external_memory_fd = true,
            b"VK_KHR_win32_keyed_mutex" => self.khr_win32_keyed_mutex = true,
            b"VK_KHR_external_semaphore" => self.khr_external_semaphore = true,
            b"VK_KHR_external_semaphore_win32" => self.khr_external_semaphore_win32 = true,
            b"VK_KHR_external_semaphore_fd" => self.khr_external_semaphore_fd = true,
            b"VK_KHR_push_descriptor" => self.khr_push_descriptor = true,
            b"VK_EXT_conditional_rendering" => self.ext_conditional_rendering = true,
            b"VK_KHR_shader_float16_int8" => self.khr_shader_float16_int8 = true,
            b"VK_KHR_16bit_storage" => self.khr_16bit_storage = true,
            b"VK_KHR_incremental_present" => self.khr_incremental_present = true,
            b"VK_KHR_descriptor_update_template" => self.khr_descriptor_update_template = true,
            b"VK_NV_clip_space_w_scaling" => self.nv_clip_space_w_scaling = true,
            b"VK_EXT_display_control" => self.ext_display_control = true,
            b"VK_GOOGLE_display_timing" => self.google_display_timing = true,
            b"VK_NV_sample_mask_override_coverage" => self.nv_sample_mask_override_coverage = true,
            b"VK_NV_geometry_shader_passthrough" => self.nv_geometry_shader_passthrough = true,
            b"VK_NV_viewport_array2" => self.nv_viewport_array2 = true,
            b"VK_NVX_multiview_per_view_attributes" => self.nvx_multiview_per_view_attributes = true,
            b"VK_NV_viewport_swizzle" => self.nv_viewport_swizzle = true,
            b"VK_EXT_discard_rectangles" => self.ext_discard_rectangles = true,
            b"VK_EXT_conservative_rasterization" => self.ext_conservative_rasterization = true,
            b"VK_EXT_depth_clip_enable" => self.ext_depth_clip_enable = true,
            b"VK_EXT_hdr_metadata" => self.ext_hdr_metadata = true,
            b"VK_KHR_imageless_framebuffer" => self.khr_imageless_framebuffer = true,
            b"VK_KHR_create_renderpass2" => self.khr_create_renderpass2 = true,
            b"VK_KHR_shared_presentable_image" => self.khr_shared_presentable_image = true,
            b"VK_KHR_external_fence" => self.khr_external_fence = true,
            b"VK_KHR_external_fence_win32" => self.khr_external_fence_win32 = true,
            b"VK_KHR_external_fence_fd" => self.khr_external_fence_fd = true,
            b"VK_KHR_performance_query" => self.khr_performance_query = true,
            b"VK_KHR_maintenance2" => self.khr_maintenance2 = true,
            b"VK_KHR_variable_pointers" => self.khr_variable_pointers = true,
            b"VK_EXT_external_memory_dma_buf" => self.ext_external_memory_dma_buf = true,
            b"VK_EXT_queue_family_foreign" => self.ext_queue_family_foreign = true,
            b"VK_KHR_dedicated_allocation" => self.khr_dedicated_allocation = true,
            b"VK_ANDROID_external_memory_android_hardware_buffer" => {
                self.android_external_memory_android_hardware_buffer = true
            }
            b"VK_EXT_sampler_filter_minmax" => self.ext_sampler_filter_minmax = true,
            b"VK_KHR_storage_buffer_storage_class" => self.khr_storage_buffer_storage_class = true,
            b"VK_AMD_gpu_shader_int16" => self.amd_gpu_shader_int16 = true,
            b"VK_AMD_mixed_attachment_samples" => self.amd_mixed_attachment_samples = true,
            b"VK_AMD_shader_fragment_mask" => self.amd_shader_fragment_mask = true,
            b"VK_EXT_inline_uniform_block" => self.ext_inline_uniform_block = true,
            b"VK_EXT_shader_stencil_export" => self.ext_shader_stencil_export = true,
            b"VK_EXT_sample_locations" => self.ext_sample_locations = true,
            b"VK_KHR_relaxed_block_layout" => self.khr_relaxed_block_layout = true,
            b"VK_KHR_get_memory_requirements2" => self.khr_get_memory_requirements2 = true,
            b"VK_KHR_image_format_list" => self.khr_image_format_list = true,
            b"VK_EXT_blend_operation_advanced" => self.ext_blend_operation_advanced = true,
            b"VK_NV_fragment_coverage_to_color" => self.nv_fragment_coverage_to_color = true,
            b"VK_KHR_acceleration_structure" => self.khr_acceleration_structure = true,
            b"VK_KHR_ray_tracing_pipeline" => self.khr_ray_tracing_pipeline = true,
            b"VK_KHR_ray_query" => self.khr_ray_query = true,
            b"VK_NV_framebuffer_mixed_samples" => self.nv_framebuffer_mixed_samples = true,
            b"VK_NV_fill_rectangle" => self.nv_fill_rectangle = true,
            b"VK_NV_shader_sm_builtins" => self.nv_shader_sm_builtins = true,
            b"VK_EXT_post_depth_coverage" => self.ext_post_depth_coverage = true,
            b"VK_KHR_sampler_ycbcr_conversion" => self.khr_sampler_ycbcr_conversion = true,
            b"VK_KHR_bind_memory2" => self.khr_bind_memory2 = true,
            b"VK_EXT_image_drm_format_modifier" => self.ext_image_drm_format_modifier = true,
            b"VK_EXT_validation_cache" => self.ext_validation_cache = true,
            b"VK_EXT_descriptor_indexing" => self.ext_descriptor_indexing = true,
            b"VK_EXT_shader_viewport_index_layer" => self.ext_shader_viewport_index_layer = true,
            b"VK_KHR_portability_subset" => self.khr_portability_subset = true,
            b"VK_NV_shading_rate_image" => self.nv_shading_rate_image = true,
            b"VK_NV_ray_tracing" => self.nv_ray_tracing = true,
            b"VK_NV_representative_fragment_test" => self.nv_representative_fragment_test = true,
            b"VK_KHR_maintenance3" => self.khr_maintenance3 = true,
            b"VK_KHR_draw_indirect_count" => self.khr_draw_indirect_count = true,
            b"VK_EXT_filter_cubic" => self.ext_filter_cubic = true,
            b"VK_QCOM_render_pass_shader_resolve" => self.qcom_render_pass_shader_resolve = true,
            b"VK_EXT_global_priority" => self.ext_global_priority = true,
            b"VK_KHR_shader_subgroup_extended_types" => self.khr_shader_subgroup_extended_types = true,
            b"VK_KHR_8bit_storage" => self.khr_8bit_storage = true,
            b"VK_EXT_external_memory_host" => self.ext_external_memory_host = true,
            b"VK_AMD_buffer_marker" => self.amd_buffer_marker = true,
            b"VK_KHR_shader_atomic_int64" => self.khr_shader_atomic_int64 = true,
            b"VK_KHR_shader_clock" => self.khr_shader_clock = true,
            b"VK_AMD_pipeline_compiler_control" => self.amd_pipeline_compiler_control = true,
            b"VK_EXT_calibrated_timestamps" => self.ext_calibrated_timestamps = true,
            b"VK_AMD_shader_core_properties" => self.amd_shader_core_properties = true,
            b"VK_AMD_memory_overallocation_behavior" => self.amd_memory_overallocation_behavior = true,
            b"VK_EXT_vertex_attribute_divisor" => self.ext_vertex_attribute_divisor = true,
            b"VK_EXT_pipeline_creation_feedback" => self.ext_pipeline_creation_feedback = true,
            b"VK_KHR_driver_properties" => self.khr_driver_properties = true,
            b"VK_KHR_shader_float_controls" => self.khr_shader_float_controls = true,
            b"VK_NV_shader_subgroup_partitioned" => self.nv_shader_subgroup_partitioned = true,
            b"VK_KHR_depth_stencil_resolve" => self.khr_depth_stencil_resolve = true,
            b"VK_KHR_swapchain_mutable_format" => self.khr_swapchain_mutable_format = true,
            b"VK_NV_compute_shader_derivatives" => self.nv_compute_shader_derivatives = true,
            b"VK_NV_mesh_shader" => self.nv_mesh_shader = true,
            b"VK_NV_fragment_shader_barycentric" => self.nv_fragment_shader_barycentric = true,
            b"VK_NV_shader_image_footprint" => self.nv_shader_image_footprint = true,
            b"VK_NV_scissor_exclusive" => self.nv_scissor_exclusive = true,
            b"VK_NV_device_diagnostic_checkpoints" => self.nv_device_diagnostic_checkpoints = true,
            b"VK_KHR_timeline_semaphore" => self.khr_timeline_semaphore = true,
            b"VK_INTEL_shader_integer_functions2" => self.intel_shader_integer_functions2 = true,
            b"VK_INTEL_performance_query" => self.intel_performance_query = true,
            b"VK_KHR_vulkan_memory_model" => self.khr_vulkan_memory_model = true,
            b"VK_EXT_pci_bus_info" => self.ext_pci_bus_info = true,
            b"VK_AMD_display_native_hdr" => self.amd_display_native_hdr = true,
            b"VK_KHR_shader_terminate_invocation" => self.khr_shader_terminate_invocation = true,
            b"VK_EXT_fragment_density_map" => self.ext_fragment_density_map = true,
            b"VK_EXT_scalar_block_layout" => self.ext_scalar_block_layout = true,
            b"VK_GOOGLE_hlsl_functionality1" => self.google_hlsl_functionality1 = true,
            b"VK_GOOGLE_decorate_string" => self.google_decorate_string = true,
            b"VK_EXT_subgroup_size_control" => self.ext_subgroup_size_control = true,
            b"VK_KHR_fragment_shading_rate" => self.khr_fragment_shading_rate = true,
            b"VK_AMD_shader_core_properties2" => self.amd_shader_core_properties2 = true,
            b"VK_AMD_device_coherent_memory" => self.amd_device_coherent_memory = true,
            b"VK_EXT_shader_image_atomic_int64" => self.ext_shader_image_atomic_int64 = true,
            b"VK_KHR_spirv_1_4" => self.khr_spirv_1_4 = true,
            b"VK_EXT_memory_budget" => self.ext_memory_budget = true,
            b"VK_EXT_memory_priority" => self.ext_memory_priority = true,
            b"VK_NV_dedicated_allocation_image_aliasing" => self.nv_dedicated_allocation_image_aliasing = true,
            b"VK_KHR_separate_depth_stencil_layouts" => self.khr_separate_depth_stencil_layouts = true,
            b"VK_EXT_buffer_device_address" => self.ext_buffer_device_address = true,
            b"VK_EXT_tooling_info" => self.ext_tooling_info = true,
            b"VK_EXT_separate_stencil_usage" => self.ext_separate_stencil_usage = true,
            b"VK_NV_cooperative_matrix" => self.nv_cooperative_matrix = true,
            b"VK_NV_coverage_reduction_mode" => self.nv_coverage_reduction_mode = true,
            b"VK_EXT_fragment_shader_interlock" => self.ext_fragment_shader_interlock = true,
            b"VK_EXT_ycbcr_image_arrays" => self.ext_ycbcr_image_arrays = true,
            b"VK_KHR_uniform_buffer_standard_layout" => self.khr_uniform_buffer_standard_layout = true,
            b"VK_EXT_full_screen_exclusive" => self.ext_full_screen_exclusive = true,
            b"VK_KHR_buffer_device_address" => self.khr_buffer_device_address = true,
            b"VK_EXT_line_rasterization" => self.ext_line_rasterization = true,
            b"VK_EXT_shader_atomic_float" => self.ext_shader_atomic_float = true,
            b"VK_EXT_host_query_reset" => self.ext_host_query_reset = true,
            b"VK_EXT_index_type_uint8" => self.ext_index_type_uint8 = true,
            b"VK_EXT_extended_dynamic_state" => self.ext_extended_dynamic_state = true,
            b"VK_KHR_deferred_host_operations" => self.khr_deferred_host_operations = true,
            b"VK_KHR_pipeline_executable_properties" => self.khr_pipeline_executable_properties = true,
            b"VK_EXT_shader_demote_to_helper_invocation" => self.ext_shader_demote_to_helper_invocation = true,
            b"VK_NV_device_generated_commands" => self.nv_device_generated_commands = true,
            b"VK_NV_inherited_viewport_scissor" => self.nv_inherited_viewport_scissor = true,
            b"VK_EXT_texel_buffer_alignment" => self.ext_texel_buffer_alignment = true,
            b"VK_QCOM_render_pass_transform" => self.qcom_render_pass_transform = true,
            b"VK_EXT_device_memory_report" => self.ext_device_memory_report = true,
            b"VK_EXT_robustness2" => self.ext_robustness2 = true,
            b"VK_EXT_custom_border_color" => self.ext_custom_border_color = true,
            b"VK_GOOGLE_user_type" => self.google_user_type = true,
            b"VK_KHR_pipeline_library" => self.khr_pipeline_library = true,
            b"VK_KHR_shader_non_semantic_info" => self.khr_shader_non_semantic_info = true,
            b"VK_EXT_private_data" => self.ext_private_data = true,
            b"VK_EXT_pipeline_creation_cache_control" => self.ext_pipeline_creation_cache_control = true,
            b"VK_NV_device_diagnostics_config" => self.nv_device_diagnostics_config = true,
            b"VK_QCOM_render_pass_store_ops" => self.qcom_render_pass_store_ops = true,
            b"VK_KHR_synchronization2" => self.khr_synchronization2 = true,
            b"VK_KHR_zero_initialize_workgroup_memory" => self.khr_zero_initialize_workgroup_memory = true,
            b"VK_NV_fragment_shading_rate_enums" => self.nv_fragment_shading_rate_enums = true,
            b"VK_EXT_ycbcr_2plane_444_formats" => self.ext_ycbcr_2plane_444_formats = true,
            b"VK_EXT_fragment_density_map2" => self.ext_fragment_density_map2 = true,
            b"VK_QCOM_rotated_copy_commands" => self.qcom_rotated_copy_commands = true,
            b"VK_EXT_image_robustness" => self.ext_image_robustness = true,
            b"VK_KHR_workgroup_memory_explicit_layout" => self.khr_workgroup_memory_explicit_layout = true,
            b"VK_KHR_copy_commands2" => self.khr_copy_commands2 = true,
            b"VK_EXT_4444_formats" => self.ext_4444_formats = true,
            b"VK_NV_acquire_winrt_display" => self.nv_acquire_winrt_display = true,
            b"VK_VALVE_mutable_descriptor_type" => self.valve_mutable_descriptor_type = true,
            b"VK_EXT_vertex_input_dynamic_state" => self.ext_vertex_input_dynamic_state = true,
            b"VK_FUCHSIA_external_memory" => self.fuchsia_external_memory = true,
            b"VK_FUCHSIA_external_semaphore" => self.fuchsia_external_semaphore = true,
            b"VK_EXT_color_write_enable" => self.ext_color_write_enable = true,
            _ => {}
        }
    }
    pub fn new(core_version: vk::Version) -> Self {
        Self {
            core_version,
            khr_swapchain: false,
            khr_display_swapchain: false,
            nv_glsl_shader: false,
            ext_depth_range_unrestricted: false,
            khr_sampler_mirror_clamp_to_edge: false,
            img_filter_cubic: false,
            amd_rasterization_order: false,
            amd_shader_trinary_minmax: false,
            amd_shader_explicit_vertex_parameter: false,
            ext_debug_marker: false,
            amd_gcn_shader: false,
            nv_dedicated_allocation: false,
            ext_transform_feedback: false,
            nvx_image_view_handle: false,
            amd_draw_indirect_count: false,
            amd_negative_viewport_height: false,
            amd_gpu_shader_half_float: false,
            amd_shader_ballot: false,
            amd_texture_gather_bias_lod: false,
            amd_shader_info: false,
            amd_shader_image_load_store_lod: false,
            nv_corner_sampled_image: false,
            khr_multiview: false,
            img_format_pvrtc: false,
            nv_external_memory: false,
            nv_external_memory_win32: false,
            nv_win32_keyed_mutex: false,
            khr_device_group: false,
            khr_shader_draw_parameters: false,
            ext_shader_subgroup_ballot: false,
            ext_shader_subgroup_vote: false,
            ext_texture_compression_astc_hdr: false,
            ext_astc_decode_mode: false,
            khr_maintenance1: false,
            khr_external_memory: false,
            khr_external_memory_win32: false,
            khr_external_memory_fd: false,
            khr_win32_keyed_mutex: false,
            khr_external_semaphore: false,
            khr_external_semaphore_win32: false,
            khr_external_semaphore_fd: false,
            khr_push_descriptor: false,
            ext_conditional_rendering: false,
            khr_shader_float16_int8: false,
            khr_16bit_storage: false,
            khr_incremental_present: false,
            khr_descriptor_update_template: false,
            nv_clip_space_w_scaling: false,
            ext_display_control: false,
            google_display_timing: false,
            nv_sample_mask_override_coverage: false,
            nv_geometry_shader_passthrough: false,
            nv_viewport_array2: false,
            nvx_multiview_per_view_attributes: false,
            nv_viewport_swizzle: false,
            ext_discard_rectangles: false,
            ext_conservative_rasterization: false,
            ext_depth_clip_enable: false,
            ext_hdr_metadata: false,
            khr_imageless_framebuffer: false,
            khr_create_renderpass2: false,
            khr_shared_presentable_image: false,
            khr_external_fence: false,
            khr_external_fence_win32: false,
            khr_external_fence_fd: false,
            khr_performance_query: false,
            khr_maintenance2: false,
            khr_variable_pointers: false,
            ext_external_memory_dma_buf: false,
            ext_queue_family_foreign: false,
            khr_dedicated_allocation: false,
            android_external_memory_android_hardware_buffer: false,
            ext_sampler_filter_minmax: false,
            khr_storage_buffer_storage_class: false,
            amd_gpu_shader_int16: false,
            amd_mixed_attachment_samples: false,
            amd_shader_fragment_mask: false,
            ext_inline_uniform_block: false,
            ext_shader_stencil_export: false,
            ext_sample_locations: false,
            khr_relaxed_block_layout: false,
            khr_get_memory_requirements2: false,
            khr_image_format_list: false,
            ext_blend_operation_advanced: false,
            nv_fragment_coverage_to_color: false,
            khr_acceleration_structure: false,
            khr_ray_tracing_pipeline: false,
            khr_ray_query: false,
            nv_framebuffer_mixed_samples: false,
            nv_fill_rectangle: false,
            nv_shader_sm_builtins: false,
            ext_post_depth_coverage: false,
            khr_sampler_ycbcr_conversion: false,
            khr_bind_memory2: false,
            ext_image_drm_format_modifier: false,
            ext_validation_cache: false,
            ext_descriptor_indexing: false,
            ext_shader_viewport_index_layer: false,
            khr_portability_subset: false,
            nv_shading_rate_image: false,
            nv_ray_tracing: false,
            nv_representative_fragment_test: false,
            khr_maintenance3: false,
            khr_draw_indirect_count: false,
            ext_filter_cubic: false,
            qcom_render_pass_shader_resolve: false,
            ext_global_priority: false,
            khr_shader_subgroup_extended_types: false,
            khr_8bit_storage: false,
            ext_external_memory_host: false,
            amd_buffer_marker: false,
            khr_shader_atomic_int64: false,
            khr_shader_clock: false,
            amd_pipeline_compiler_control: false,
            ext_calibrated_timestamps: false,
            amd_shader_core_properties: false,
            amd_memory_overallocation_behavior: false,
            ext_vertex_attribute_divisor: false,
            ext_pipeline_creation_feedback: false,
            khr_driver_properties: false,
            khr_shader_float_controls: false,
            nv_shader_subgroup_partitioned: false,
            khr_depth_stencil_resolve: false,
            khr_swapchain_mutable_format: false,
            nv_compute_shader_derivatives: false,
            nv_mesh_shader: false,
            nv_fragment_shader_barycentric: false,
            nv_shader_image_footprint: false,
            nv_scissor_exclusive: false,
            nv_device_diagnostic_checkpoints: false,
            khr_timeline_semaphore: false,
            intel_shader_integer_functions2: false,
            intel_performance_query: false,
            khr_vulkan_memory_model: false,
            ext_pci_bus_info: false,
            amd_display_native_hdr: false,
            khr_shader_terminate_invocation: false,
            ext_fragment_density_map: false,
            ext_scalar_block_layout: false,
            google_hlsl_functionality1: false,
            google_decorate_string: false,
            ext_subgroup_size_control: false,
            khr_fragment_shading_rate: false,
            amd_shader_core_properties2: false,
            amd_device_coherent_memory: false,
            ext_shader_image_atomic_int64: false,
            khr_spirv_1_4: false,
            ext_memory_budget: false,
            ext_memory_priority: false,
            nv_dedicated_allocation_image_aliasing: false,
            khr_separate_depth_stencil_layouts: false,
            ext_buffer_device_address: false,
            ext_tooling_info: false,
            ext_separate_stencil_usage: false,
            nv_cooperative_matrix: false,
            nv_coverage_reduction_mode: false,
            ext_fragment_shader_interlock: false,
            ext_ycbcr_image_arrays: false,
            khr_uniform_buffer_standard_layout: false,
            ext_full_screen_exclusive: false,
            khr_buffer_device_address: false,
            ext_line_rasterization: false,
            ext_shader_atomic_float: false,
            ext_host_query_reset: false,
            ext_index_type_uint8: false,
            ext_extended_dynamic_state: false,
            khr_deferred_host_operations: false,
            khr_pipeline_executable_properties: false,
            ext_shader_demote_to_helper_invocation: false,
            nv_device_generated_commands: false,
            nv_inherited_viewport_scissor: false,
            ext_texel_buffer_alignment: false,
            qcom_render_pass_transform: false,
            ext_device_memory_report: false,
            ext_robustness2: false,
            ext_custom_border_color: false,
            google_user_type: false,
            khr_pipeline_library: false,
            khr_shader_non_semantic_info: false,
            ext_private_data: false,
            ext_pipeline_creation_cache_control: false,
            nv_device_diagnostics_config: false,
            qcom_render_pass_store_ops: false,
            khr_synchronization2: false,
            khr_zero_initialize_workgroup_memory: false,
            nv_fragment_shading_rate_enums: false,
            ext_ycbcr_2plane_444_formats: false,
            ext_fragment_density_map2: false,
            qcom_rotated_copy_commands: false,
            ext_image_robustness: false,
            khr_workgroup_memory_explicit_layout: false,
            khr_copy_commands2: false,
            ext_4444_formats: false,
            nv_acquire_winrt_display: false,
            valve_mutable_descriptor_type: false,
            ext_vertex_input_dynamic_state: false,
            fuchsia_external_memory: false,
            fuchsia_external_semaphore: false,
            ext_color_write_enable: false,
        }
    }
    pub fn from_properties(core_version: vk::Version, properties: &[vk::ExtensionProperties]) -> Self {
        let mut ext = Self::new(core_version);
        for ep in properties.iter() {
            if ep.extension_name.iter().any(|&c| c == 0) {
                let name = unsafe { CStr::from_ptr(ep.extension_name.as_ptr()) };
                ext.enable_by_name(name);
            }
        }
        ext
    }
    pub fn supports_khr_swapchain(&self) -> bool {
        self.khr_swapchain
    }
    pub fn enable_khr_swapchain(&mut self) {
        self.khr_swapchain = true;
    }
    pub fn supports_khr_display_swapchain(&self) -> bool {
        self.khr_display_swapchain && self.supports_khr_swapchain()
    }
    pub fn enable_khr_display_swapchain(&mut self) {
        self.khr_display_swapchain = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_nv_glsl_shader(&self) -> bool {
        self.nv_glsl_shader
    }
    pub fn enable_nv_glsl_shader(&mut self) {
        self.nv_glsl_shader = true;
    }
    pub fn supports_ext_depth_range_unrestricted(&self) -> bool {
        self.ext_depth_range_unrestricted
    }
    pub fn enable_ext_depth_range_unrestricted(&mut self) {
        self.ext_depth_range_unrestricted = true;
    }
    pub fn supports_khr_sampler_mirror_clamp_to_edge(&self) -> bool {
        self.khr_sampler_mirror_clamp_to_edge || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_sampler_mirror_clamp_to_edge(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_sampler_mirror_clamp_to_edge = true;
        }
    }
    pub fn supports_img_filter_cubic(&self) -> bool {
        self.img_filter_cubic
    }
    pub fn enable_img_filter_cubic(&mut self) {
        self.img_filter_cubic = true;
    }
    pub fn supports_amd_rasterization_order(&self) -> bool {
        self.amd_rasterization_order
    }
    pub fn enable_amd_rasterization_order(&mut self) {
        self.amd_rasterization_order = true;
    }
    pub fn supports_amd_shader_trinary_minmax(&self) -> bool {
        self.amd_shader_trinary_minmax
    }
    pub fn enable_amd_shader_trinary_minmax(&mut self) {
        self.amd_shader_trinary_minmax = true;
    }
    pub fn supports_amd_shader_explicit_vertex_parameter(&self) -> bool {
        self.amd_shader_explicit_vertex_parameter
    }
    pub fn enable_amd_shader_explicit_vertex_parameter(&mut self) {
        self.amd_shader_explicit_vertex_parameter = true;
    }
    pub fn supports_ext_debug_marker(&self) -> bool {
        self.ext_debug_marker
    }
    pub fn enable_ext_debug_marker(&mut self) {
        self.ext_debug_marker = true;
    }
    pub fn supports_amd_gcn_shader(&self) -> bool {
        self.amd_gcn_shader
    }
    pub fn enable_amd_gcn_shader(&mut self) {
        self.amd_gcn_shader = true;
    }
    pub fn supports_nv_dedicated_allocation(&self) -> bool {
        self.nv_dedicated_allocation
    }
    pub fn enable_nv_dedicated_allocation(&mut self) {
        self.nv_dedicated_allocation = true;
    }
    pub fn supports_ext_transform_feedback(&self) -> bool {
        self.ext_transform_feedback
    }
    pub fn enable_ext_transform_feedback(&mut self) {
        self.ext_transform_feedback = true;
    }
    pub fn supports_nvx_image_view_handle(&self) -> bool {
        self.nvx_image_view_handle
    }
    pub fn enable_nvx_image_view_handle(&mut self) {
        self.nvx_image_view_handle = true;
    }
    pub fn supports_amd_draw_indirect_count(&self) -> bool {
        self.amd_draw_indirect_count
    }
    pub fn enable_amd_draw_indirect_count(&mut self) {
        self.amd_draw_indirect_count = true;
    }
    pub fn supports_amd_negative_viewport_height(&self) -> bool {
        self.amd_negative_viewport_height
    }
    pub fn enable_amd_negative_viewport_height(&mut self) {
        self.amd_negative_viewport_height = true;
    }
    pub fn supports_amd_gpu_shader_half_float(&self) -> bool {
        self.amd_gpu_shader_half_float
    }
    pub fn enable_amd_gpu_shader_half_float(&mut self) {
        self.amd_gpu_shader_half_float = true;
    }
    pub fn supports_amd_shader_ballot(&self) -> bool {
        self.amd_shader_ballot
    }
    pub fn enable_amd_shader_ballot(&mut self) {
        self.amd_shader_ballot = true;
    }
    pub fn supports_amd_texture_gather_bias_lod(&self) -> bool {
        self.amd_texture_gather_bias_lod
    }
    pub fn enable_amd_texture_gather_bias_lod(&mut self) {
        self.amd_texture_gather_bias_lod = true;
    }
    pub fn supports_amd_shader_info(&self) -> bool {
        self.amd_shader_info
    }
    pub fn enable_amd_shader_info(&mut self) {
        self.amd_shader_info = true;
    }
    pub fn supports_amd_shader_image_load_store_lod(&self) -> bool {
        self.amd_shader_image_load_store_lod
    }
    pub fn enable_amd_shader_image_load_store_lod(&mut self) {
        self.amd_shader_image_load_store_lod = true;
    }
    pub fn supports_nv_corner_sampled_image(&self) -> bool {
        self.nv_corner_sampled_image
    }
    pub fn enable_nv_corner_sampled_image(&mut self) {
        self.nv_corner_sampled_image = true;
    }
    pub fn supports_khr_multiview(&self) -> bool {
        self.khr_multiview || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_multiview(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_multiview = true;
        }
    }
    pub fn supports_img_format_pvrtc(&self) -> bool {
        self.img_format_pvrtc
    }
    pub fn enable_img_format_pvrtc(&mut self) {
        self.img_format_pvrtc = true;
    }
    pub fn supports_nv_external_memory(&self) -> bool {
        self.nv_external_memory
    }
    pub fn enable_nv_external_memory(&mut self) {
        self.nv_external_memory = true;
    }
    pub fn supports_nv_external_memory_win32(&self) -> bool {
        self.nv_external_memory_win32 && self.supports_nv_external_memory()
    }
    pub fn enable_nv_external_memory_win32(&mut self) {
        self.nv_external_memory_win32 = true;
        self.enable_nv_external_memory();
    }
    pub fn supports_nv_win32_keyed_mutex(&self) -> bool {
        self.nv_win32_keyed_mutex && self.supports_nv_external_memory_win32() && self.supports_nv_external_memory()
    }
    pub fn enable_nv_win32_keyed_mutex(&mut self) {
        self.nv_win32_keyed_mutex = true;
        self.enable_nv_external_memory_win32();
        self.enable_nv_external_memory();
    }
    pub fn supports_khr_device_group(&self) -> bool {
        self.khr_device_group || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_device_group(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_device_group = true;
        }
    }
    pub fn supports_khr_shader_draw_parameters(&self) -> bool {
        self.khr_shader_draw_parameters || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_shader_draw_parameters(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_shader_draw_parameters = true;
        }
    }
    pub fn supports_ext_shader_subgroup_ballot(&self) -> bool {
        self.ext_shader_subgroup_ballot
    }
    pub fn enable_ext_shader_subgroup_ballot(&mut self) {
        self.ext_shader_subgroup_ballot = true;
    }
    pub fn supports_ext_shader_subgroup_vote(&self) -> bool {
        self.ext_shader_subgroup_vote
    }
    pub fn enable_ext_shader_subgroup_vote(&mut self) {
        self.ext_shader_subgroup_vote = true;
    }
    pub fn supports_ext_texture_compression_astc_hdr(&self) -> bool {
        self.ext_texture_compression_astc_hdr
    }
    pub fn enable_ext_texture_compression_astc_hdr(&mut self) {
        self.ext_texture_compression_astc_hdr = true;
    }
    pub fn supports_ext_astc_decode_mode(&self) -> bool {
        self.ext_astc_decode_mode
    }
    pub fn enable_ext_astc_decode_mode(&mut self) {
        self.ext_astc_decode_mode = true;
    }
    pub fn supports_khr_maintenance1(&self) -> bool {
        self.khr_maintenance1 || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_maintenance1(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_maintenance1 = true;
        }
    }
    pub fn supports_khr_external_memory(&self) -> bool {
        self.khr_external_memory || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_external_memory(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_external_memory = true;
        }
    }
    pub fn supports_khr_external_memory_win32(&self) -> bool {
        self.khr_external_memory_win32 && self.supports_khr_external_memory()
    }
    pub fn enable_khr_external_memory_win32(&mut self) {
        self.khr_external_memory_win32 = true;
        self.enable_khr_external_memory();
    }
    pub fn supports_khr_external_memory_fd(&self) -> bool {
        self.khr_external_memory_fd && self.supports_khr_external_memory()
    }
    pub fn enable_khr_external_memory_fd(&mut self) {
        self.khr_external_memory_fd = true;
        self.enable_khr_external_memory();
    }
    pub fn supports_khr_win32_keyed_mutex(&self) -> bool {
        self.khr_win32_keyed_mutex && self.supports_khr_external_memory_win32() && self.supports_khr_external_memory()
    }
    pub fn enable_khr_win32_keyed_mutex(&mut self) {
        self.khr_win32_keyed_mutex = true;
        self.enable_khr_external_memory_win32();
        self.enable_khr_external_memory();
    }
    pub fn supports_khr_external_semaphore(&self) -> bool {
        self.khr_external_semaphore || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_external_semaphore(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_external_semaphore = true;
        }
    }
    pub fn supports_khr_external_semaphore_win32(&self) -> bool {
        self.khr_external_semaphore_win32 && self.supports_khr_external_semaphore()
    }
    pub fn enable_khr_external_semaphore_win32(&mut self) {
        self.khr_external_semaphore_win32 = true;
        self.enable_khr_external_semaphore();
    }
    pub fn supports_khr_external_semaphore_fd(&self) -> bool {
        self.khr_external_semaphore_fd && self.supports_khr_external_semaphore()
    }
    pub fn enable_khr_external_semaphore_fd(&mut self) {
        self.khr_external_semaphore_fd = true;
        self.enable_khr_external_semaphore();
    }
    pub fn supports_khr_push_descriptor(&self) -> bool {
        self.khr_push_descriptor
    }
    pub fn enable_khr_push_descriptor(&mut self) {
        self.khr_push_descriptor = true;
    }
    pub fn supports_ext_conditional_rendering(&self) -> bool {
        self.ext_conditional_rendering
    }
    pub fn enable_ext_conditional_rendering(&mut self) {
        self.ext_conditional_rendering = true;
    }
    pub fn supports_khr_shader_float16_int8(&self) -> bool {
        self.khr_shader_float16_int8 || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_shader_float16_int8(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_shader_float16_int8 = true;
        }
    }
    pub fn supports_khr_16bit_storage(&self) -> bool {
        (self.khr_16bit_storage || self.core_version >= vk::Version::from_raw_parts(1, 1, 0))
            && self.supports_khr_storage_buffer_storage_class()
    }
    pub fn enable_khr_16bit_storage(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_16bit_storage = true;
        }
        self.enable_khr_storage_buffer_storage_class();
    }
    pub fn supports_khr_incremental_present(&self) -> bool {
        self.khr_incremental_present && self.supports_khr_swapchain()
    }
    pub fn enable_khr_incremental_present(&mut self) {
        self.khr_incremental_present = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_khr_descriptor_update_template(&self) -> bool {
        self.khr_descriptor_update_template || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_descriptor_update_template(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_descriptor_update_template = true;
        }
    }
    pub fn supports_nv_clip_space_w_scaling(&self) -> bool {
        self.nv_clip_space_w_scaling
    }
    pub fn enable_nv_clip_space_w_scaling(&mut self) {
        self.nv_clip_space_w_scaling = true;
    }
    pub fn supports_ext_display_control(&self) -> bool {
        self.ext_display_control && self.supports_khr_swapchain()
    }
    pub fn enable_ext_display_control(&mut self) {
        self.ext_display_control = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_google_display_timing(&self) -> bool {
        self.google_display_timing && self.supports_khr_swapchain()
    }
    pub fn enable_google_display_timing(&mut self) {
        self.google_display_timing = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_nv_sample_mask_override_coverage(&self) -> bool {
        self.nv_sample_mask_override_coverage
    }
    pub fn enable_nv_sample_mask_override_coverage(&mut self) {
        self.nv_sample_mask_override_coverage = true;
    }
    pub fn supports_nv_geometry_shader_passthrough(&self) -> bool {
        self.nv_geometry_shader_passthrough
    }
    pub fn enable_nv_geometry_shader_passthrough(&mut self) {
        self.nv_geometry_shader_passthrough = true;
    }
    pub fn supports_nv_viewport_array2(&self) -> bool {
        self.nv_viewport_array2
    }
    pub fn enable_nv_viewport_array2(&mut self) {
        self.nv_viewport_array2 = true;
    }
    pub fn supports_nvx_multiview_per_view_attributes(&self) -> bool {
        self.nvx_multiview_per_view_attributes && self.supports_khr_multiview()
    }
    pub fn enable_nvx_multiview_per_view_attributes(&mut self) {
        self.nvx_multiview_per_view_attributes = true;
        self.enable_khr_multiview();
    }
    pub fn supports_nv_viewport_swizzle(&self) -> bool {
        self.nv_viewport_swizzle
    }
    pub fn enable_nv_viewport_swizzle(&mut self) {
        self.nv_viewport_swizzle = true;
    }
    pub fn supports_ext_discard_rectangles(&self) -> bool {
        self.ext_discard_rectangles
    }
    pub fn enable_ext_discard_rectangles(&mut self) {
        self.ext_discard_rectangles = true;
    }
    pub fn supports_ext_conservative_rasterization(&self) -> bool {
        self.ext_conservative_rasterization
    }
    pub fn enable_ext_conservative_rasterization(&mut self) {
        self.ext_conservative_rasterization = true;
    }
    pub fn supports_ext_depth_clip_enable(&self) -> bool {
        self.ext_depth_clip_enable
    }
    pub fn enable_ext_depth_clip_enable(&mut self) {
        self.ext_depth_clip_enable = true;
    }
    pub fn supports_ext_hdr_metadata(&self) -> bool {
        self.ext_hdr_metadata && self.supports_khr_swapchain()
    }
    pub fn enable_ext_hdr_metadata(&mut self) {
        self.ext_hdr_metadata = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_khr_imageless_framebuffer(&self) -> bool {
        (self.khr_imageless_framebuffer || self.core_version >= vk::Version::from_raw_parts(1, 2, 0))
            && self.supports_khr_maintenance2()
            && self.supports_khr_image_format_list()
    }
    pub fn enable_khr_imageless_framebuffer(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_imageless_framebuffer = true;
        }
        self.enable_khr_maintenance2();
        self.enable_khr_image_format_list();
    }
    pub fn supports_khr_create_renderpass2(&self) -> bool {
        (self.khr_create_renderpass2 || self.core_version >= vk::Version::from_raw_parts(1, 2, 0))
            && self.supports_khr_multiview()
            && self.supports_khr_maintenance2()
    }
    pub fn enable_khr_create_renderpass2(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_create_renderpass2 = true;
        }
        self.enable_khr_multiview();
        self.enable_khr_maintenance2();
    }
    pub fn supports_khr_shared_presentable_image(&self) -> bool {
        self.khr_shared_presentable_image && self.supports_khr_swapchain()
    }
    pub fn enable_khr_shared_presentable_image(&mut self) {
        self.khr_shared_presentable_image = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_khr_external_fence(&self) -> bool {
        self.khr_external_fence || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_external_fence(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_external_fence = true;
        }
    }
    pub fn supports_khr_external_fence_win32(&self) -> bool {
        self.khr_external_fence_win32 && self.supports_khr_external_fence()
    }
    pub fn enable_khr_external_fence_win32(&mut self) {
        self.khr_external_fence_win32 = true;
        self.enable_khr_external_fence();
    }
    pub fn supports_khr_external_fence_fd(&self) -> bool {
        self.khr_external_fence_fd && self.supports_khr_external_fence()
    }
    pub fn enable_khr_external_fence_fd(&mut self) {
        self.khr_external_fence_fd = true;
        self.enable_khr_external_fence();
    }
    pub fn supports_khr_performance_query(&self) -> bool {
        self.khr_performance_query
    }
    pub fn enable_khr_performance_query(&mut self) {
        self.khr_performance_query = true;
    }
    pub fn supports_khr_maintenance2(&self) -> bool {
        self.khr_maintenance2 || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_maintenance2(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_maintenance2 = true;
        }
    }
    pub fn supports_khr_variable_pointers(&self) -> bool {
        (self.khr_variable_pointers || self.core_version >= vk::Version::from_raw_parts(1, 1, 0))
            && self.supports_khr_storage_buffer_storage_class()
    }
    pub fn enable_khr_variable_pointers(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_variable_pointers = true;
        }
        self.enable_khr_storage_buffer_storage_class();
    }
    pub fn supports_ext_external_memory_dma_buf(&self) -> bool {
        self.ext_external_memory_dma_buf
            && self.supports_khr_external_memory_fd()
            && self.supports_khr_external_memory()
    }
    pub fn enable_ext_external_memory_dma_buf(&mut self) {
        self.ext_external_memory_dma_buf = true;
        self.enable_khr_external_memory_fd();
        self.enable_khr_external_memory();
    }
    pub fn supports_ext_queue_family_foreign(&self) -> bool {
        self.ext_queue_family_foreign && self.supports_khr_external_memory()
    }
    pub fn enable_ext_queue_family_foreign(&mut self) {
        self.ext_queue_family_foreign = true;
        self.enable_khr_external_memory();
    }
    pub fn supports_khr_dedicated_allocation(&self) -> bool {
        (self.khr_dedicated_allocation || self.core_version >= vk::Version::from_raw_parts(1, 1, 0))
            && self.supports_khr_get_memory_requirements2()
    }
    pub fn enable_khr_dedicated_allocation(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_dedicated_allocation = true;
        }
        self.enable_khr_get_memory_requirements2();
    }
    pub fn supports_android_external_memory_android_hardware_buffer(&self) -> bool {
        self.android_external_memory_android_hardware_buffer
            && self.supports_khr_sampler_ycbcr_conversion()
            && self.supports_khr_external_memory()
            && self.supports_ext_queue_family_foreign()
            && self.supports_khr_dedicated_allocation()
            && self.supports_khr_maintenance1()
            && self.supports_khr_bind_memory2()
            && self.supports_khr_get_memory_requirements2()
    }
    pub fn enable_android_external_memory_android_hardware_buffer(&mut self) {
        self.android_external_memory_android_hardware_buffer = true;
        self.enable_khr_sampler_ycbcr_conversion();
        self.enable_khr_external_memory();
        self.enable_ext_queue_family_foreign();
        self.enable_khr_dedicated_allocation();
        self.enable_khr_maintenance1();
        self.enable_khr_bind_memory2();
        self.enable_khr_get_memory_requirements2();
    }
    pub fn supports_ext_sampler_filter_minmax(&self) -> bool {
        self.ext_sampler_filter_minmax || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_ext_sampler_filter_minmax(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.ext_sampler_filter_minmax = true;
        }
    }
    pub fn supports_khr_storage_buffer_storage_class(&self) -> bool {
        self.khr_storage_buffer_storage_class || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_storage_buffer_storage_class(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_storage_buffer_storage_class = true;
        }
    }
    pub fn supports_amd_gpu_shader_int16(&self) -> bool {
        self.amd_gpu_shader_int16
    }
    pub fn enable_amd_gpu_shader_int16(&mut self) {
        self.amd_gpu_shader_int16 = true;
    }
    pub fn supports_amd_mixed_attachment_samples(&self) -> bool {
        self.amd_mixed_attachment_samples
    }
    pub fn enable_amd_mixed_attachment_samples(&mut self) {
        self.amd_mixed_attachment_samples = true;
    }
    pub fn supports_amd_shader_fragment_mask(&self) -> bool {
        self.amd_shader_fragment_mask
    }
    pub fn enable_amd_shader_fragment_mask(&mut self) {
        self.amd_shader_fragment_mask = true;
    }
    pub fn supports_ext_inline_uniform_block(&self) -> bool {
        self.ext_inline_uniform_block && self.supports_khr_maintenance1()
    }
    pub fn enable_ext_inline_uniform_block(&mut self) {
        self.ext_inline_uniform_block = true;
        self.enable_khr_maintenance1();
    }
    pub fn supports_ext_shader_stencil_export(&self) -> bool {
        self.ext_shader_stencil_export
    }
    pub fn enable_ext_shader_stencil_export(&mut self) {
        self.ext_shader_stencil_export = true;
    }
    pub fn supports_ext_sample_locations(&self) -> bool {
        self.ext_sample_locations
    }
    pub fn enable_ext_sample_locations(&mut self) {
        self.ext_sample_locations = true;
    }
    pub fn supports_khr_relaxed_block_layout(&self) -> bool {
        self.khr_relaxed_block_layout || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_relaxed_block_layout(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_relaxed_block_layout = true;
        }
    }
    pub fn supports_khr_get_memory_requirements2(&self) -> bool {
        self.khr_get_memory_requirements2 || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_get_memory_requirements2(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_get_memory_requirements2 = true;
        }
    }
    pub fn supports_khr_image_format_list(&self) -> bool {
        self.khr_image_format_list || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_image_format_list(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_image_format_list = true;
        }
    }
    pub fn supports_ext_blend_operation_advanced(&self) -> bool {
        self.ext_blend_operation_advanced
    }
    pub fn enable_ext_blend_operation_advanced(&mut self) {
        self.ext_blend_operation_advanced = true;
    }
    pub fn supports_nv_fragment_coverage_to_color(&self) -> bool {
        self.nv_fragment_coverage_to_color
    }
    pub fn enable_nv_fragment_coverage_to_color(&mut self) {
        self.nv_fragment_coverage_to_color = true;
    }
    pub fn supports_khr_acceleration_structure(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
            && self.khr_acceleration_structure
            && self.supports_ext_descriptor_indexing()
            && self.supports_khr_buffer_device_address()
            && self.supports_khr_deferred_host_operations()
            && self.supports_khr_maintenance3()
    }
    pub fn enable_khr_acceleration_structure(&mut self) {
        self.khr_acceleration_structure = true;
        self.enable_ext_descriptor_indexing();
        self.enable_khr_buffer_device_address();
        self.enable_khr_deferred_host_operations();
        self.enable_khr_maintenance3();
    }
    pub fn supports_khr_ray_tracing_pipeline(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
            && self.khr_ray_tracing_pipeline
            && self.supports_khr_spirv_1_4()
            && self.supports_khr_acceleration_structure()
            && self.supports_khr_shader_float_controls()
            && self.supports_ext_descriptor_indexing()
            && self.supports_khr_buffer_device_address()
            && self.supports_khr_deferred_host_operations()
            && self.supports_khr_maintenance3()
    }
    pub fn enable_khr_ray_tracing_pipeline(&mut self) {
        self.khr_ray_tracing_pipeline = true;
        self.enable_khr_spirv_1_4();
        self.enable_khr_acceleration_structure();
        self.enable_khr_shader_float_controls();
        self.enable_ext_descriptor_indexing();
        self.enable_khr_buffer_device_address();
        self.enable_khr_deferred_host_operations();
        self.enable_khr_maintenance3();
    }
    pub fn supports_khr_ray_query(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
            && self.khr_ray_query
            && self.supports_khr_spirv_1_4()
            && self.supports_khr_acceleration_structure()
            && self.supports_khr_shader_float_controls()
            && self.supports_ext_descriptor_indexing()
            && self.supports_khr_buffer_device_address()
            && self.supports_khr_deferred_host_operations()
            && self.supports_khr_maintenance3()
    }
    pub fn enable_khr_ray_query(&mut self) {
        self.khr_ray_query = true;
        self.enable_khr_spirv_1_4();
        self.enable_khr_acceleration_structure();
        self.enable_khr_shader_float_controls();
        self.enable_ext_descriptor_indexing();
        self.enable_khr_buffer_device_address();
        self.enable_khr_deferred_host_operations();
        self.enable_khr_maintenance3();
    }
    pub fn supports_nv_framebuffer_mixed_samples(&self) -> bool {
        self.nv_framebuffer_mixed_samples
    }
    pub fn enable_nv_framebuffer_mixed_samples(&mut self) {
        self.nv_framebuffer_mixed_samples = true;
    }
    pub fn supports_nv_fill_rectangle(&self) -> bool {
        self.nv_fill_rectangle
    }
    pub fn enable_nv_fill_rectangle(&mut self) {
        self.nv_fill_rectangle = true;
    }
    pub fn supports_nv_shader_sm_builtins(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0) && self.nv_shader_sm_builtins
    }
    pub fn enable_nv_shader_sm_builtins(&mut self) {
        self.nv_shader_sm_builtins = true;
    }
    pub fn supports_ext_post_depth_coverage(&self) -> bool {
        self.ext_post_depth_coverage
    }
    pub fn enable_ext_post_depth_coverage(&mut self) {
        self.ext_post_depth_coverage = true;
    }
    pub fn supports_khr_sampler_ycbcr_conversion(&self) -> bool {
        (self.khr_sampler_ycbcr_conversion || self.core_version >= vk::Version::from_raw_parts(1, 1, 0))
            && self.supports_khr_maintenance1()
            && self.supports_khr_bind_memory2()
            && self.supports_khr_get_memory_requirements2()
    }
    pub fn enable_khr_sampler_ycbcr_conversion(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_sampler_ycbcr_conversion = true;
        }
        self.enable_khr_maintenance1();
        self.enable_khr_bind_memory2();
        self.enable_khr_get_memory_requirements2();
    }
    pub fn supports_khr_bind_memory2(&self) -> bool {
        self.khr_bind_memory2 || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_bind_memory2(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_bind_memory2 = true;
        }
    }
    pub fn supports_ext_image_drm_format_modifier(&self) -> bool {
        self.ext_image_drm_format_modifier
            && self.supports_khr_bind_memory2()
            && self.supports_khr_image_format_list()
            && self.supports_khr_sampler_ycbcr_conversion()
            && self.supports_khr_maintenance1()
            && self.supports_khr_get_memory_requirements2()
    }
    pub fn enable_ext_image_drm_format_modifier(&mut self) {
        self.ext_image_drm_format_modifier = true;
        self.enable_khr_bind_memory2();
        self.enable_khr_image_format_list();
        self.enable_khr_sampler_ycbcr_conversion();
        self.enable_khr_maintenance1();
        self.enable_khr_get_memory_requirements2();
    }
    pub fn supports_ext_validation_cache(&self) -> bool {
        self.ext_validation_cache
    }
    pub fn enable_ext_validation_cache(&mut self) {
        self.ext_validation_cache = true;
    }
    pub fn supports_ext_descriptor_indexing(&self) -> bool {
        (self.ext_descriptor_indexing || self.core_version >= vk::Version::from_raw_parts(1, 2, 0))
            && self.supports_khr_maintenance3()
    }
    pub fn enable_ext_descriptor_indexing(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.ext_descriptor_indexing = true;
        }
        self.enable_khr_maintenance3();
    }
    pub fn supports_ext_shader_viewport_index_layer(&self) -> bool {
        self.ext_shader_viewport_index_layer || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_ext_shader_viewport_index_layer(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.ext_shader_viewport_index_layer = true;
        }
    }
    pub fn supports_khr_portability_subset(&self) -> bool {
        self.khr_portability_subset
    }
    pub fn enable_khr_portability_subset(&mut self) {
        self.khr_portability_subset = true;
    }
    pub fn supports_nv_shading_rate_image(&self) -> bool {
        self.nv_shading_rate_image
    }
    pub fn enable_nv_shading_rate_image(&mut self) {
        self.nv_shading_rate_image = true;
    }
    pub fn supports_nv_ray_tracing(&self) -> bool {
        self.nv_ray_tracing && self.supports_khr_get_memory_requirements2()
    }
    pub fn enable_nv_ray_tracing(&mut self) {
        self.nv_ray_tracing = true;
        self.enable_khr_get_memory_requirements2();
    }
    pub fn supports_nv_representative_fragment_test(&self) -> bool {
        self.nv_representative_fragment_test
    }
    pub fn enable_nv_representative_fragment_test(&mut self) {
        self.nv_representative_fragment_test = true;
    }
    pub fn supports_khr_maintenance3(&self) -> bool {
        self.khr_maintenance3 || self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
    }
    pub fn enable_khr_maintenance3(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 1, 0) {
            self.khr_maintenance3 = true;
        }
    }
    pub fn supports_khr_draw_indirect_count(&self) -> bool {
        self.khr_draw_indirect_count || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_draw_indirect_count(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_draw_indirect_count = true;
        }
    }
    pub fn supports_ext_filter_cubic(&self) -> bool {
        self.ext_filter_cubic
    }
    pub fn enable_ext_filter_cubic(&mut self) {
        self.ext_filter_cubic = true;
    }
    pub fn supports_qcom_render_pass_shader_resolve(&self) -> bool {
        self.qcom_render_pass_shader_resolve
    }
    pub fn enable_qcom_render_pass_shader_resolve(&mut self) {
        self.qcom_render_pass_shader_resolve = true;
    }
    pub fn supports_ext_global_priority(&self) -> bool {
        self.ext_global_priority
    }
    pub fn enable_ext_global_priority(&mut self) {
        self.ext_global_priority = true;
    }
    pub fn supports_khr_shader_subgroup_extended_types(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
            && (self.khr_shader_subgroup_extended_types || self.core_version >= vk::Version::from_raw_parts(1, 2, 0))
    }
    pub fn enable_khr_shader_subgroup_extended_types(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_shader_subgroup_extended_types = true;
        }
    }
    pub fn supports_khr_8bit_storage(&self) -> bool {
        (self.khr_8bit_storage || self.core_version >= vk::Version::from_raw_parts(1, 2, 0))
            && self.supports_khr_storage_buffer_storage_class()
    }
    pub fn enable_khr_8bit_storage(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_8bit_storage = true;
        }
        self.enable_khr_storage_buffer_storage_class();
    }
    pub fn supports_ext_external_memory_host(&self) -> bool {
        self.ext_external_memory_host && self.supports_khr_external_memory()
    }
    pub fn enable_ext_external_memory_host(&mut self) {
        self.ext_external_memory_host = true;
        self.enable_khr_external_memory();
    }
    pub fn supports_amd_buffer_marker(&self) -> bool {
        self.amd_buffer_marker
    }
    pub fn enable_amd_buffer_marker(&mut self) {
        self.amd_buffer_marker = true;
    }
    pub fn supports_khr_shader_atomic_int64(&self) -> bool {
        self.khr_shader_atomic_int64 || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_shader_atomic_int64(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_shader_atomic_int64 = true;
        }
    }
    pub fn supports_khr_shader_clock(&self) -> bool {
        self.khr_shader_clock
    }
    pub fn enable_khr_shader_clock(&mut self) {
        self.khr_shader_clock = true;
    }
    pub fn supports_amd_pipeline_compiler_control(&self) -> bool {
        self.amd_pipeline_compiler_control
    }
    pub fn enable_amd_pipeline_compiler_control(&mut self) {
        self.amd_pipeline_compiler_control = true;
    }
    pub fn supports_ext_calibrated_timestamps(&self) -> bool {
        self.ext_calibrated_timestamps
    }
    pub fn enable_ext_calibrated_timestamps(&mut self) {
        self.ext_calibrated_timestamps = true;
    }
    pub fn supports_amd_shader_core_properties(&self) -> bool {
        self.amd_shader_core_properties
    }
    pub fn enable_amd_shader_core_properties(&mut self) {
        self.amd_shader_core_properties = true;
    }
    pub fn supports_amd_memory_overallocation_behavior(&self) -> bool {
        self.amd_memory_overallocation_behavior
    }
    pub fn enable_amd_memory_overallocation_behavior(&mut self) {
        self.amd_memory_overallocation_behavior = true;
    }
    pub fn supports_ext_vertex_attribute_divisor(&self) -> bool {
        self.ext_vertex_attribute_divisor
    }
    pub fn enable_ext_vertex_attribute_divisor(&mut self) {
        self.ext_vertex_attribute_divisor = true;
    }
    pub fn supports_ext_pipeline_creation_feedback(&self) -> bool {
        self.ext_pipeline_creation_feedback
    }
    pub fn enable_ext_pipeline_creation_feedback(&mut self) {
        self.ext_pipeline_creation_feedback = true;
    }
    pub fn supports_khr_driver_properties(&self) -> bool {
        self.khr_driver_properties || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_driver_properties(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_driver_properties = true;
        }
    }
    pub fn supports_khr_shader_float_controls(&self) -> bool {
        self.khr_shader_float_controls || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_shader_float_controls(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_shader_float_controls = true;
        }
    }
    pub fn supports_nv_shader_subgroup_partitioned(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0) && self.nv_shader_subgroup_partitioned
    }
    pub fn enable_nv_shader_subgroup_partitioned(&mut self) {
        self.nv_shader_subgroup_partitioned = true;
    }
    pub fn supports_khr_depth_stencil_resolve(&self) -> bool {
        (self.khr_depth_stencil_resolve || self.core_version >= vk::Version::from_raw_parts(1, 2, 0))
            && self.supports_khr_create_renderpass2()
            && self.supports_khr_multiview()
            && self.supports_khr_maintenance2()
    }
    pub fn enable_khr_depth_stencil_resolve(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_depth_stencil_resolve = true;
        }
        self.enable_khr_create_renderpass2();
        self.enable_khr_multiview();
        self.enable_khr_maintenance2();
    }
    pub fn supports_khr_swapchain_mutable_format(&self) -> bool {
        self.khr_swapchain_mutable_format
            && self.supports_khr_swapchain()
            && self.supports_khr_maintenance2()
            && self.supports_khr_image_format_list()
    }
    pub fn enable_khr_swapchain_mutable_format(&mut self) {
        self.khr_swapchain_mutable_format = true;
        self.enable_khr_swapchain();
        self.enable_khr_maintenance2();
        self.enable_khr_image_format_list();
    }
    pub fn supports_nv_compute_shader_derivatives(&self) -> bool {
        self.nv_compute_shader_derivatives
    }
    pub fn enable_nv_compute_shader_derivatives(&mut self) {
        self.nv_compute_shader_derivatives = true;
    }
    pub fn supports_nv_mesh_shader(&self) -> bool {
        self.nv_mesh_shader
    }
    pub fn enable_nv_mesh_shader(&mut self) {
        self.nv_mesh_shader = true;
    }
    pub fn supports_nv_fragment_shader_barycentric(&self) -> bool {
        self.nv_fragment_shader_barycentric
    }
    pub fn enable_nv_fragment_shader_barycentric(&mut self) {
        self.nv_fragment_shader_barycentric = true;
    }
    pub fn supports_nv_shader_image_footprint(&self) -> bool {
        self.nv_shader_image_footprint
    }
    pub fn enable_nv_shader_image_footprint(&mut self) {
        self.nv_shader_image_footprint = true;
    }
    pub fn supports_nv_scissor_exclusive(&self) -> bool {
        self.nv_scissor_exclusive
    }
    pub fn enable_nv_scissor_exclusive(&mut self) {
        self.nv_scissor_exclusive = true;
    }
    pub fn supports_nv_device_diagnostic_checkpoints(&self) -> bool {
        self.nv_device_diagnostic_checkpoints
    }
    pub fn enable_nv_device_diagnostic_checkpoints(&mut self) {
        self.nv_device_diagnostic_checkpoints = true;
    }
    pub fn supports_khr_timeline_semaphore(&self) -> bool {
        self.khr_timeline_semaphore || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_timeline_semaphore(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_timeline_semaphore = true;
        }
    }
    pub fn supports_intel_shader_integer_functions2(&self) -> bool {
        self.intel_shader_integer_functions2
    }
    pub fn enable_intel_shader_integer_functions2(&mut self) {
        self.intel_shader_integer_functions2 = true;
    }
    pub fn supports_intel_performance_query(&self) -> bool {
        self.intel_performance_query
    }
    pub fn enable_intel_performance_query(&mut self) {
        self.intel_performance_query = true;
    }
    pub fn supports_khr_vulkan_memory_model(&self) -> bool {
        self.khr_vulkan_memory_model || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_vulkan_memory_model(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_vulkan_memory_model = true;
        }
    }
    pub fn supports_ext_pci_bus_info(&self) -> bool {
        self.ext_pci_bus_info
    }
    pub fn enable_ext_pci_bus_info(&mut self) {
        self.ext_pci_bus_info = true;
    }
    pub fn supports_amd_display_native_hdr(&self) -> bool {
        self.amd_display_native_hdr && self.supports_khr_swapchain()
    }
    pub fn enable_amd_display_native_hdr(&mut self) {
        self.amd_display_native_hdr = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_khr_shader_terminate_invocation(&self) -> bool {
        self.khr_shader_terminate_invocation
    }
    pub fn enable_khr_shader_terminate_invocation(&mut self) {
        self.khr_shader_terminate_invocation = true;
    }
    pub fn supports_ext_fragment_density_map(&self) -> bool {
        self.ext_fragment_density_map
    }
    pub fn enable_ext_fragment_density_map(&mut self) {
        self.ext_fragment_density_map = true;
    }
    pub fn supports_ext_scalar_block_layout(&self) -> bool {
        self.ext_scalar_block_layout || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_ext_scalar_block_layout(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.ext_scalar_block_layout = true;
        }
    }
    pub fn supports_google_hlsl_functionality1(&self) -> bool {
        self.google_hlsl_functionality1
    }
    pub fn enable_google_hlsl_functionality1(&mut self) {
        self.google_hlsl_functionality1 = true;
    }
    pub fn supports_google_decorate_string(&self) -> bool {
        self.google_decorate_string
    }
    pub fn enable_google_decorate_string(&mut self) {
        self.google_decorate_string = true;
    }
    pub fn supports_ext_subgroup_size_control(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0) && self.ext_subgroup_size_control
    }
    pub fn enable_ext_subgroup_size_control(&mut self) {
        self.ext_subgroup_size_control = true;
    }
    pub fn supports_khr_fragment_shading_rate(&self) -> bool {
        self.khr_fragment_shading_rate
            && self.supports_khr_create_renderpass2()
            && self.supports_khr_multiview()
            && self.supports_khr_maintenance2()
    }
    pub fn enable_khr_fragment_shading_rate(&mut self) {
        self.khr_fragment_shading_rate = true;
        self.enable_khr_create_renderpass2();
        self.enable_khr_multiview();
        self.enable_khr_maintenance2();
    }
    pub fn supports_amd_shader_core_properties2(&self) -> bool {
        self.amd_shader_core_properties2 && self.supports_amd_shader_core_properties()
    }
    pub fn enable_amd_shader_core_properties2(&mut self) {
        self.amd_shader_core_properties2 = true;
        self.enable_amd_shader_core_properties();
    }
    pub fn supports_amd_device_coherent_memory(&self) -> bool {
        self.amd_device_coherent_memory
    }
    pub fn enable_amd_device_coherent_memory(&mut self) {
        self.amd_device_coherent_memory = true;
    }
    pub fn supports_ext_shader_image_atomic_int64(&self) -> bool {
        self.ext_shader_image_atomic_int64
    }
    pub fn enable_ext_shader_image_atomic_int64(&mut self) {
        self.ext_shader_image_atomic_int64 = true;
    }
    pub fn supports_khr_spirv_1_4(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0)
            && (self.khr_spirv_1_4 || self.core_version >= vk::Version::from_raw_parts(1, 2, 0))
            && self.supports_khr_shader_float_controls()
    }
    pub fn enable_khr_spirv_1_4(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_spirv_1_4 = true;
        }
        self.enable_khr_shader_float_controls();
    }
    pub fn supports_ext_memory_budget(&self) -> bool {
        self.ext_memory_budget
    }
    pub fn enable_ext_memory_budget(&mut self) {
        self.ext_memory_budget = true;
    }
    pub fn supports_ext_memory_priority(&self) -> bool {
        self.ext_memory_priority
    }
    pub fn enable_ext_memory_priority(&mut self) {
        self.ext_memory_priority = true;
    }
    pub fn supports_nv_dedicated_allocation_image_aliasing(&self) -> bool {
        self.nv_dedicated_allocation_image_aliasing
            && self.supports_khr_dedicated_allocation()
            && self.supports_khr_get_memory_requirements2()
    }
    pub fn enable_nv_dedicated_allocation_image_aliasing(&mut self) {
        self.nv_dedicated_allocation_image_aliasing = true;
        self.enable_khr_dedicated_allocation();
        self.enable_khr_get_memory_requirements2();
    }
    pub fn supports_khr_separate_depth_stencil_layouts(&self) -> bool {
        (self.khr_separate_depth_stencil_layouts || self.core_version >= vk::Version::from_raw_parts(1, 2, 0))
            && self.supports_khr_create_renderpass2()
            && self.supports_khr_multiview()
            && self.supports_khr_maintenance2()
    }
    pub fn enable_khr_separate_depth_stencil_layouts(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_separate_depth_stencil_layouts = true;
        }
        self.enable_khr_create_renderpass2();
        self.enable_khr_multiview();
        self.enable_khr_maintenance2();
    }
    pub fn supports_ext_buffer_device_address(&self) -> bool {
        self.ext_buffer_device_address
    }
    pub fn enable_ext_buffer_device_address(&mut self) {
        self.ext_buffer_device_address = true;
    }
    pub fn supports_ext_tooling_info(&self) -> bool {
        self.ext_tooling_info
    }
    pub fn enable_ext_tooling_info(&mut self) {
        self.ext_tooling_info = true;
    }
    pub fn supports_ext_separate_stencil_usage(&self) -> bool {
        self.ext_separate_stencil_usage || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_ext_separate_stencil_usage(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.ext_separate_stencil_usage = true;
        }
    }
    pub fn supports_nv_cooperative_matrix(&self) -> bool {
        self.nv_cooperative_matrix
    }
    pub fn enable_nv_cooperative_matrix(&mut self) {
        self.nv_cooperative_matrix = true;
    }
    pub fn supports_nv_coverage_reduction_mode(&self) -> bool {
        self.nv_coverage_reduction_mode && self.supports_nv_framebuffer_mixed_samples()
    }
    pub fn enable_nv_coverage_reduction_mode(&mut self) {
        self.nv_coverage_reduction_mode = true;
        self.enable_nv_framebuffer_mixed_samples();
    }
    pub fn supports_ext_fragment_shader_interlock(&self) -> bool {
        self.ext_fragment_shader_interlock
    }
    pub fn enable_ext_fragment_shader_interlock(&mut self) {
        self.ext_fragment_shader_interlock = true;
    }
    pub fn supports_ext_ycbcr_image_arrays(&self) -> bool {
        self.ext_ycbcr_image_arrays
            && self.supports_khr_sampler_ycbcr_conversion()
            && self.supports_khr_maintenance1()
            && self.supports_khr_bind_memory2()
            && self.supports_khr_get_memory_requirements2()
    }
    pub fn enable_ext_ycbcr_image_arrays(&mut self) {
        self.ext_ycbcr_image_arrays = true;
        self.enable_khr_sampler_ycbcr_conversion();
        self.enable_khr_maintenance1();
        self.enable_khr_bind_memory2();
        self.enable_khr_get_memory_requirements2();
    }
    pub fn supports_khr_uniform_buffer_standard_layout(&self) -> bool {
        self.khr_uniform_buffer_standard_layout || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_uniform_buffer_standard_layout(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_uniform_buffer_standard_layout = true;
        }
    }
    pub fn supports_ext_full_screen_exclusive(&self) -> bool {
        self.ext_full_screen_exclusive && self.supports_khr_swapchain()
    }
    pub fn enable_ext_full_screen_exclusive(&mut self) {
        self.ext_full_screen_exclusive = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_khr_buffer_device_address(&self) -> bool {
        self.khr_buffer_device_address || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_khr_buffer_device_address(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.khr_buffer_device_address = true;
        }
    }
    pub fn supports_ext_line_rasterization(&self) -> bool {
        self.ext_line_rasterization
    }
    pub fn enable_ext_line_rasterization(&mut self) {
        self.ext_line_rasterization = true;
    }
    pub fn supports_ext_shader_atomic_float(&self) -> bool {
        self.ext_shader_atomic_float
    }
    pub fn enable_ext_shader_atomic_float(&mut self) {
        self.ext_shader_atomic_float = true;
    }
    pub fn supports_ext_host_query_reset(&self) -> bool {
        self.ext_host_query_reset || self.core_version >= vk::Version::from_raw_parts(1, 2, 0)
    }
    pub fn enable_ext_host_query_reset(&mut self) {
        if self.core_version < vk::Version::from_raw_parts(1, 2, 0) {
            self.ext_host_query_reset = true;
        }
    }
    pub fn supports_ext_index_type_uint8(&self) -> bool {
        self.ext_index_type_uint8
    }
    pub fn enable_ext_index_type_uint8(&mut self) {
        self.ext_index_type_uint8 = true;
    }
    pub fn supports_ext_extended_dynamic_state(&self) -> bool {
        self.ext_extended_dynamic_state
    }
    pub fn enable_ext_extended_dynamic_state(&mut self) {
        self.ext_extended_dynamic_state = true;
    }
    pub fn supports_khr_deferred_host_operations(&self) -> bool {
        self.khr_deferred_host_operations
    }
    pub fn enable_khr_deferred_host_operations(&mut self) {
        self.khr_deferred_host_operations = true;
    }
    pub fn supports_khr_pipeline_executable_properties(&self) -> bool {
        self.khr_pipeline_executable_properties
    }
    pub fn enable_khr_pipeline_executable_properties(&mut self) {
        self.khr_pipeline_executable_properties = true;
    }
    pub fn supports_ext_shader_demote_to_helper_invocation(&self) -> bool {
        self.ext_shader_demote_to_helper_invocation
    }
    pub fn enable_ext_shader_demote_to_helper_invocation(&mut self) {
        self.ext_shader_demote_to_helper_invocation = true;
    }
    pub fn supports_nv_device_generated_commands(&self) -> bool {
        self.core_version >= vk::Version::from_raw_parts(1, 1, 0) && self.nv_device_generated_commands
    }
    pub fn enable_nv_device_generated_commands(&mut self) {
        self.nv_device_generated_commands = true;
    }
    pub fn supports_nv_inherited_viewport_scissor(&self) -> bool {
        self.nv_inherited_viewport_scissor
    }
    pub fn enable_nv_inherited_viewport_scissor(&mut self) {
        self.nv_inherited_viewport_scissor = true;
    }
    pub fn supports_ext_texel_buffer_alignment(&self) -> bool {
        self.ext_texel_buffer_alignment
    }
    pub fn enable_ext_texel_buffer_alignment(&mut self) {
        self.ext_texel_buffer_alignment = true;
    }
    pub fn supports_qcom_render_pass_transform(&self) -> bool {
        self.qcom_render_pass_transform && self.supports_khr_swapchain()
    }
    pub fn enable_qcom_render_pass_transform(&mut self) {
        self.qcom_render_pass_transform = true;
        self.enable_khr_swapchain();
    }
    pub fn supports_ext_device_memory_report(&self) -> bool {
        self.ext_device_memory_report
    }
    pub fn enable_ext_device_memory_report(&mut self) {
        self.ext_device_memory_report = true;
    }
    pub fn supports_ext_robustness2(&self) -> bool {
        self.ext_robustness2
    }
    pub fn enable_ext_robustness2(&mut self) {
        self.ext_robustness2 = true;
    }
    pub fn supports_ext_custom_border_color(&self) -> bool {
        self.ext_custom_border_color
    }
    pub fn enable_ext_custom_border_color(&mut self) {
        self.ext_custom_border_color = true;
    }
    pub fn supports_google_user_type(&self) -> bool {
        self.google_user_type
    }
    pub fn enable_google_user_type(&mut self) {
        self.google_user_type = true;
    }
    pub fn supports_khr_pipeline_library(&self) -> bool {
        self.khr_pipeline_library
    }
    pub fn enable_khr_pipeline_library(&mut self) {
        self.khr_pipeline_library = true;
    }
    pub fn supports_khr_shader_non_semantic_info(&self) -> bool {
        self.khr_shader_non_semantic_info
    }
    pub fn enable_khr_shader_non_semantic_info(&mut self) {
        self.khr_shader_non_semantic_info = true;
    }
    pub fn supports_ext_private_data(&self) -> bool {
        self.ext_private_data
    }
    pub fn enable_ext_private_data(&mut self) {
        self.ext_private_data = true;
    }
    pub fn supports_ext_pipeline_creation_cache_control(&self) -> bool {
        self.ext_pipeline_creation_cache_control
    }
    pub fn enable_ext_pipeline_creation_cache_control(&mut self) {
        self.ext_pipeline_creation_cache_control = true;
    }
    pub fn supports_nv_device_diagnostics_config(&self) -> bool {
        self.nv_device_diagnostics_config
    }
    pub fn enable_nv_device_diagnostics_config(&mut self) {
        self.nv_device_diagnostics_config = true;
    }
    pub fn supports_qcom_render_pass_store_ops(&self) -> bool {
        self.qcom_render_pass_store_ops
    }
    pub fn enable_qcom_render_pass_store_ops(&mut self) {
        self.qcom_render_pass_store_ops = true;
    }
    pub fn supports_khr_synchronization2(&self) -> bool {
        self.khr_synchronization2
    }
    pub fn enable_khr_synchronization2(&mut self) {
        self.khr_synchronization2 = true;
    }
    pub fn supports_khr_zero_initialize_workgroup_memory(&self) -> bool {
        self.khr_zero_initialize_workgroup_memory
    }
    pub fn enable_khr_zero_initialize_workgroup_memory(&mut self) {
        self.khr_zero_initialize_workgroup_memory = true;
    }
    pub fn supports_nv_fragment_shading_rate_enums(&self) -> bool {
        self.nv_fragment_shading_rate_enums
            && self.supports_khr_fragment_shading_rate()
            && self.supports_khr_create_renderpass2()
            && self.supports_khr_multiview()
            && self.supports_khr_maintenance2()
    }
    pub fn enable_nv_fragment_shading_rate_enums(&mut self) {
        self.nv_fragment_shading_rate_enums = true;
        self.enable_khr_fragment_shading_rate();
        self.enable_khr_create_renderpass2();
        self.enable_khr_multiview();
        self.enable_khr_maintenance2();
    }
    pub fn supports_ext_ycbcr_2plane_444_formats(&self) -> bool {
        self.ext_ycbcr_2plane_444_formats
            && self.supports_khr_sampler_ycbcr_conversion()
            && self.supports_khr_maintenance1()
            && self.supports_khr_bind_memory2()
            && self.supports_khr_get_memory_requirements2()
    }
    pub fn enable_ext_ycbcr_2plane_444_formats(&mut self) {
        self.ext_ycbcr_2plane_444_formats = true;
        self.enable_khr_sampler_ycbcr_conversion();
        self.enable_khr_maintenance1();
        self.enable_khr_bind_memory2();
        self.enable_khr_get_memory_requirements2();
    }
    pub fn supports_ext_fragment_density_map2(&self) -> bool {
        self.ext_fragment_density_map2 && self.supports_ext_fragment_density_map()
    }
    pub fn enable_ext_fragment_density_map2(&mut self) {
        self.ext_fragment_density_map2 = true;
        self.enable_ext_fragment_density_map();
    }
    pub fn supports_qcom_rotated_copy_commands(&self) -> bool {
        self.qcom_rotated_copy_commands && self.supports_khr_swapchain() && self.supports_khr_copy_commands2()
    }
    pub fn enable_qcom_rotated_copy_commands(&mut self) {
        self.qcom_rotated_copy_commands = true;
        self.enable_khr_swapchain();
        self.enable_khr_copy_commands2();
    }
    pub fn supports_ext_image_robustness(&self) -> bool {
        self.ext_image_robustness
    }
    pub fn enable_ext_image_robustness(&mut self) {
        self.ext_image_robustness = true;
    }
    pub fn supports_khr_workgroup_memory_explicit_layout(&self) -> bool {
        self.khr_workgroup_memory_explicit_layout
    }
    pub fn enable_khr_workgroup_memory_explicit_layout(&mut self) {
        self.khr_workgroup_memory_explicit_layout = true;
    }
    pub fn supports_khr_copy_commands2(&self) -> bool {
        self.khr_copy_commands2
    }
    pub fn enable_khr_copy_commands2(&mut self) {
        self.khr_copy_commands2 = true;
    }
    pub fn supports_ext_4444_formats(&self) -> bool {
        self.ext_4444_formats
    }
    pub fn enable_ext_4444_formats(&mut self) {
        self.ext_4444_formats = true;
    }
    pub fn supports_nv_acquire_winrt_display(&self) -> bool {
        self.nv_acquire_winrt_display
    }
    pub fn enable_nv_acquire_winrt_display(&mut self) {
        self.nv_acquire_winrt_display = true;
    }
    pub fn supports_valve_mutable_descriptor_type(&self) -> bool {
        self.valve_mutable_descriptor_type && self.supports_khr_maintenance3()
    }
    pub fn enable_valve_mutable_descriptor_type(&mut self) {
        self.valve_mutable_descriptor_type = true;
        self.enable_khr_maintenance3();
    }
    pub fn supports_ext_vertex_input_dynamic_state(&self) -> bool {
        self.ext_vertex_input_dynamic_state
    }
    pub fn enable_ext_vertex_input_dynamic_state(&mut self) {
        self.ext_vertex_input_dynamic_state = true;
    }
    pub fn supports_fuchsia_external_memory(&self) -> bool {
        self.fuchsia_external_memory && self.supports_khr_external_memory()
    }
    pub fn enable_fuchsia_external_memory(&mut self) {
        self.fuchsia_external_memory = true;
        self.enable_khr_external_memory();
    }
    pub fn supports_fuchsia_external_semaphore(&self) -> bool {
        self.fuchsia_external_semaphore && self.supports_khr_external_semaphore()
    }
    pub fn enable_fuchsia_external_semaphore(&mut self) {
        self.fuchsia_external_semaphore = true;
        self.enable_khr_external_semaphore();
    }
    pub fn supports_ext_color_write_enable(&self) -> bool {
        self.ext_color_write_enable
    }
    pub fn enable_ext_color_write_enable(&mut self) {
        self.ext_color_write_enable = true;
    }
    pub fn to_name_vec(&self) -> Vec<&'static CStr> {
        let mut v = Vec::new();
        if self.khr_swapchain {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0") })
        }
        if self.khr_display_swapchain {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display_swapchain\0") })
        }
        if self.nv_glsl_shader {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_glsl_shader\0") })
        }
        if self.ext_depth_range_unrestricted {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_range_unrestricted\0") })
        }
        if self.khr_sampler_mirror_clamp_to_edge {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_sampler_mirror_clamp_to_edge\0") })
        }
        if self.img_filter_cubic {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_IMG_filter_cubic\0") })
        }
        if self.amd_rasterization_order {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_rasterization_order\0") })
        }
        if self.amd_shader_trinary_minmax {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_trinary_minmax\0") })
        }
        if self.amd_shader_explicit_vertex_parameter {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_explicit_vertex_parameter\0") })
        }
        if self.ext_debug_marker {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_marker\0") })
        }
        if self.amd_gcn_shader {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gcn_shader\0") })
        }
        if self.nv_dedicated_allocation {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_dedicated_allocation\0") })
        }
        if self.ext_transform_feedback {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_transform_feedback\0") })
        }
        if self.nvx_image_view_handle {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NVX_image_view_handle\0") })
        }
        if self.amd_draw_indirect_count {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_draw_indirect_count\0") })
        }
        if self.amd_negative_viewport_height {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_negative_viewport_height\0") })
        }
        if self.amd_gpu_shader_half_float {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_half_float\0") })
        }
        if self.amd_shader_ballot {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_ballot\0") })
        }
        if self.amd_texture_gather_bias_lod {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_texture_gather_bias_lod\0") })
        }
        if self.amd_shader_info {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_info\0") })
        }
        if self.amd_shader_image_load_store_lod {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_image_load_store_lod\0") })
        }
        if self.nv_corner_sampled_image {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_corner_sampled_image\0") })
        }
        if self.khr_multiview {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_multiview\0") })
        }
        if self.img_format_pvrtc {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_IMG_format_pvrtc\0") })
        }
        if self.nv_external_memory {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory\0") })
        }
        if self.nv_external_memory_win32 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_win32\0") })
        }
        if self.nv_win32_keyed_mutex {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_win32_keyed_mutex\0") })
        }
        if self.khr_device_group {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_device_group\0") })
        }
        if self.khr_shader_draw_parameters {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_draw_parameters\0") })
        }
        if self.ext_shader_subgroup_ballot {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_ballot\0") })
        }
        if self.ext_shader_subgroup_vote {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_vote\0") })
        }
        if self.ext_texture_compression_astc_hdr {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_texture_compression_astc_hdr\0") })
        }
        if self.ext_astc_decode_mode {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_astc_decode_mode\0") })
        }
        if self.khr_maintenance1 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance1\0") })
        }
        if self.khr_external_memory {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory\0") })
        }
        if self.khr_external_memory_win32 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_win32\0") })
        }
        if self.khr_external_memory_fd {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_fd\0") })
        }
        if self.khr_win32_keyed_mutex {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_keyed_mutex\0") })
        }
        if self.khr_external_semaphore {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore\0") })
        }
        if self.khr_external_semaphore_win32 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_win32\0") })
        }
        if self.khr_external_semaphore_fd {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_fd\0") })
        }
        if self.khr_push_descriptor {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_push_descriptor\0") })
        }
        if self.ext_conditional_rendering {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conditional_rendering\0") })
        }
        if self.khr_shader_float16_int8 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_float16_int8\0") })
        }
        if self.khr_16bit_storage {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_16bit_storage\0") })
        }
        if self.khr_incremental_present {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_incremental_present\0") })
        }
        if self.khr_descriptor_update_template {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_descriptor_update_template\0") })
        }
        if self.nv_clip_space_w_scaling {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_clip_space_w_scaling\0") })
        }
        if self.ext_display_control {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_control\0") })
        }
        if self.google_display_timing {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_display_timing\0") })
        }
        if self.nv_sample_mask_override_coverage {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_sample_mask_override_coverage\0") })
        }
        if self.nv_geometry_shader_passthrough {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_geometry_shader_passthrough\0") })
        }
        if self.nv_viewport_array2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_array2\0") })
        }
        if self.nvx_multiview_per_view_attributes {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NVX_multiview_per_view_attributes\0") })
        }
        if self.nv_viewport_swizzle {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_swizzle\0") })
        }
        if self.ext_discard_rectangles {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_discard_rectangles\0") })
        }
        if self.ext_conservative_rasterization {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conservative_rasterization\0") })
        }
        if self.ext_depth_clip_enable {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_enable\0") })
        }
        if self.ext_hdr_metadata {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_hdr_metadata\0") })
        }
        if self.khr_imageless_framebuffer {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_imageless_framebuffer\0") })
        }
        if self.khr_create_renderpass2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_create_renderpass2\0") })
        }
        if self.khr_shared_presentable_image {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shared_presentable_image\0") })
        }
        if self.khr_external_fence {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence\0") })
        }
        if self.khr_external_fence_win32 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_win32\0") })
        }
        if self.khr_external_fence_fd {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_fd\0") })
        }
        if self.khr_performance_query {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_performance_query\0") })
        }
        if self.khr_maintenance2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance2\0") })
        }
        if self.khr_variable_pointers {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_variable_pointers\0") })
        }
        if self.ext_external_memory_dma_buf {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_dma_buf\0") })
        }
        if self.ext_queue_family_foreign {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_queue_family_foreign\0") })
        }
        if self.khr_dedicated_allocation {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_dedicated_allocation\0") })
        }
        if self.android_external_memory_android_hardware_buffer {
            v.push(unsafe {
                CStr::from_bytes_with_nul_unchecked(b"VK_ANDROID_external_memory_android_hardware_buffer\0")
            })
        }
        if self.ext_sampler_filter_minmax {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_sampler_filter_minmax\0") })
        }
        if self.khr_storage_buffer_storage_class {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_storage_buffer_storage_class\0") })
        }
        if self.amd_gpu_shader_int16 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_int16\0") })
        }
        if self.amd_mixed_attachment_samples {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_mixed_attachment_samples\0") })
        }
        if self.amd_shader_fragment_mask {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_fragment_mask\0") })
        }
        if self.ext_inline_uniform_block {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_inline_uniform_block\0") })
        }
        if self.ext_shader_stencil_export {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_stencil_export\0") })
        }
        if self.ext_sample_locations {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_sample_locations\0") })
        }
        if self.khr_relaxed_block_layout {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_relaxed_block_layout\0") })
        }
        if self.khr_get_memory_requirements2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_memory_requirements2\0") })
        }
        if self.khr_image_format_list {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_image_format_list\0") })
        }
        if self.ext_blend_operation_advanced {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_blend_operation_advanced\0") })
        }
        if self.nv_fragment_coverage_to_color {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_coverage_to_color\0") })
        }
        if self.khr_acceleration_structure {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_acceleration_structure\0") })
        }
        if self.khr_ray_tracing_pipeline {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_tracing_pipeline\0") })
        }
        if self.khr_ray_query {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_query\0") })
        }
        if self.nv_framebuffer_mixed_samples {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_framebuffer_mixed_samples\0") })
        }
        if self.nv_fill_rectangle {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_fill_rectangle\0") })
        }
        if self.nv_shader_sm_builtins {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_sm_builtins\0") })
        }
        if self.ext_post_depth_coverage {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_post_depth_coverage\0") })
        }
        if self.khr_sampler_ycbcr_conversion {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_sampler_ycbcr_conversion\0") })
        }
        if self.khr_bind_memory2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_bind_memory2\0") })
        }
        if self.ext_image_drm_format_modifier {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_drm_format_modifier\0") })
        }
        if self.ext_validation_cache {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_cache\0") })
        }
        if self.ext_descriptor_indexing {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_descriptor_indexing\0") })
        }
        if self.ext_shader_viewport_index_layer {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_viewport_index_layer\0") })
        }
        if self.khr_portability_subset {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_subset\0") })
        }
        if self.nv_shading_rate_image {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_shading_rate_image\0") })
        }
        if self.nv_ray_tracing {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing\0") })
        }
        if self.nv_representative_fragment_test {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_representative_fragment_test\0") })
        }
        if self.khr_maintenance3 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance3\0") })
        }
        if self.khr_draw_indirect_count {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_draw_indirect_count\0") })
        }
        if self.ext_filter_cubic {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_filter_cubic\0") })
        }
        if self.qcom_render_pass_shader_resolve {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_shader_resolve\0") })
        }
        if self.ext_global_priority {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_global_priority\0") })
        }
        if self.khr_shader_subgroup_extended_types {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_subgroup_extended_types\0") })
        }
        if self.khr_8bit_storage {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_8bit_storage\0") })
        }
        if self.ext_external_memory_host {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_host\0") })
        }
        if self.amd_buffer_marker {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_buffer_marker\0") })
        }
        if self.khr_shader_atomic_int64 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_atomic_int64\0") })
        }
        if self.khr_shader_clock {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_clock\0") })
        }
        if self.amd_pipeline_compiler_control {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_pipeline_compiler_control\0") })
        }
        if self.ext_calibrated_timestamps {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_calibrated_timestamps\0") })
        }
        if self.amd_shader_core_properties {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties\0") })
        }
        if self.amd_memory_overallocation_behavior {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_memory_overallocation_behavior\0") })
        }
        if self.ext_vertex_attribute_divisor {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_attribute_divisor\0") })
        }
        if self.ext_pipeline_creation_feedback {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_creation_feedback\0") })
        }
        if self.khr_driver_properties {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_driver_properties\0") })
        }
        if self.khr_shader_float_controls {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_float_controls\0") })
        }
        if self.nv_shader_subgroup_partitioned {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_subgroup_partitioned\0") })
        }
        if self.khr_depth_stencil_resolve {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_depth_stencil_resolve\0") })
        }
        if self.khr_swapchain_mutable_format {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain_mutable_format\0") })
        }
        if self.nv_compute_shader_derivatives {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_compute_shader_derivatives\0") })
        }
        if self.nv_mesh_shader {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_mesh_shader\0") })
        }
        if self.nv_fragment_shader_barycentric {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_shader_barycentric\0") })
        }
        if self.nv_shader_image_footprint {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_image_footprint\0") })
        }
        if self.nv_scissor_exclusive {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_scissor_exclusive\0") })
        }
        if self.nv_device_diagnostic_checkpoints {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_diagnostic_checkpoints\0") })
        }
        if self.khr_timeline_semaphore {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_timeline_semaphore\0") })
        }
        if self.intel_shader_integer_functions2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_shader_integer_functions2\0") })
        }
        if self.intel_performance_query {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_performance_query\0") })
        }
        if self.khr_vulkan_memory_model {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_vulkan_memory_model\0") })
        }
        if self.ext_pci_bus_info {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pci_bus_info\0") })
        }
        if self.amd_display_native_hdr {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_display_native_hdr\0") })
        }
        if self.khr_shader_terminate_invocation {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_terminate_invocation\0") })
        }
        if self.ext_fragment_density_map {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map\0") })
        }
        if self.ext_scalar_block_layout {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_scalar_block_layout\0") })
        }
        if self.google_hlsl_functionality1 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_hlsl_functionality1\0") })
        }
        if self.google_decorate_string {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_decorate_string\0") })
        }
        if self.ext_subgroup_size_control {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_subgroup_size_control\0") })
        }
        if self.khr_fragment_shading_rate {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_fragment_shading_rate\0") })
        }
        if self.amd_shader_core_properties2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties2\0") })
        }
        if self.amd_device_coherent_memory {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_AMD_device_coherent_memory\0") })
        }
        if self.ext_shader_image_atomic_int64 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_image_atomic_int64\0") })
        }
        if self.khr_spirv_1_4 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_spirv_1_4\0") })
        }
        if self.ext_memory_budget {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_budget\0") })
        }
        if self.ext_memory_priority {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_priority\0") })
        }
        if self.nv_dedicated_allocation_image_aliasing {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_dedicated_allocation_image_aliasing\0") })
        }
        if self.khr_separate_depth_stencil_layouts {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_separate_depth_stencil_layouts\0") })
        }
        if self.ext_buffer_device_address {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_buffer_device_address\0") })
        }
        if self.ext_tooling_info {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_tooling_info\0") })
        }
        if self.ext_separate_stencil_usage {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_separate_stencil_usage\0") })
        }
        if self.nv_cooperative_matrix {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_cooperative_matrix\0") })
        }
        if self.nv_coverage_reduction_mode {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_coverage_reduction_mode\0") })
        }
        if self.ext_fragment_shader_interlock {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_shader_interlock\0") })
        }
        if self.ext_ycbcr_image_arrays {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_ycbcr_image_arrays\0") })
        }
        if self.khr_uniform_buffer_standard_layout {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_uniform_buffer_standard_layout\0") })
        }
        if self.ext_full_screen_exclusive {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_full_screen_exclusive\0") })
        }
        if self.khr_buffer_device_address {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_buffer_device_address\0") })
        }
        if self.ext_line_rasterization {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_line_rasterization\0") })
        }
        if self.ext_shader_atomic_float {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float\0") })
        }
        if self.ext_host_query_reset {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_host_query_reset\0") })
        }
        if self.ext_index_type_uint8 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_index_type_uint8\0") })
        }
        if self.ext_extended_dynamic_state {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extended_dynamic_state\0") })
        }
        if self.khr_deferred_host_operations {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_deferred_host_operations\0") })
        }
        if self.khr_pipeline_executable_properties {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_pipeline_executable_properties\0") })
        }
        if self.ext_shader_demote_to_helper_invocation {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_demote_to_helper_invocation\0") })
        }
        if self.nv_device_generated_commands {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_generated_commands\0") })
        }
        if self.nv_inherited_viewport_scissor {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_inherited_viewport_scissor\0") })
        }
        if self.ext_texel_buffer_alignment {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_texel_buffer_alignment\0") })
        }
        if self.qcom_render_pass_transform {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_transform\0") })
        }
        if self.ext_device_memory_report {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_device_memory_report\0") })
        }
        if self.ext_robustness2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_robustness2\0") })
        }
        if self.ext_custom_border_color {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_custom_border_color\0") })
        }
        if self.google_user_type {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_user_type\0") })
        }
        if self.khr_pipeline_library {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_pipeline_library\0") })
        }
        if self.khr_shader_non_semantic_info {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_non_semantic_info\0") })
        }
        if self.ext_private_data {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_private_data\0") })
        }
        if self.ext_pipeline_creation_cache_control {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_creation_cache_control\0") })
        }
        if self.nv_device_diagnostics_config {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_diagnostics_config\0") })
        }
        if self.qcom_render_pass_store_ops {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_store_ops\0") })
        }
        if self.khr_synchronization2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_synchronization2\0") })
        }
        if self.khr_zero_initialize_workgroup_memory {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_zero_initialize_workgroup_memory\0") })
        }
        if self.nv_fragment_shading_rate_enums {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_shading_rate_enums\0") })
        }
        if self.ext_ycbcr_2plane_444_formats {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_ycbcr_2plane_444_formats\0") })
        }
        if self.ext_fragment_density_map2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map2\0") })
        }
        if self.qcom_rotated_copy_commands {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_rotated_copy_commands\0") })
        }
        if self.ext_image_robustness {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_robustness\0") })
        }
        if self.khr_workgroup_memory_explicit_layout {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_workgroup_memory_explicit_layout\0") })
        }
        if self.khr_copy_commands2 {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_copy_commands2\0") })
        }
        if self.ext_4444_formats {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_4444_formats\0") })
        }
        if self.nv_acquire_winrt_display {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_NV_acquire_winrt_display\0") })
        }
        if self.valve_mutable_descriptor_type {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_VALVE_mutable_descriptor_type\0") })
        }
        if self.ext_vertex_input_dynamic_state {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_input_dynamic_state\0") })
        }
        if self.fuchsia_external_memory {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_memory\0") })
        }
        if self.fuchsia_external_semaphore {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_semaphore\0") })
        }
        if self.ext_color_write_enable {
            v.push(unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_color_write_enable\0") })
        }
        v
    }
}
#[derive(Copy, Clone)]
pub struct Device {
    pub handle: vk::Device,
    pub extensions: DeviceExtensions,
    pub fp_destroy_device: Option<vk::FnDestroyDevice>,
    pub fp_get_device_queue: Option<vk::FnGetDeviceQueue>,
    pub fp_queue_submit: Option<vk::FnQueueSubmit>,
    pub fp_queue_wait_idle: Option<vk::FnQueueWaitIdle>,
    pub fp_device_wait_idle: Option<vk::FnDeviceWaitIdle>,
    pub fp_allocate_memory: Option<vk::FnAllocateMemory>,
    pub fp_free_memory: Option<vk::FnFreeMemory>,
    pub fp_map_memory: Option<vk::FnMapMemory>,
    pub fp_unmap_memory: Option<vk::FnUnmapMemory>,
    pub fp_flush_mapped_memory_ranges: Option<vk::FnFlushMappedMemoryRanges>,
    pub fp_invalidate_mapped_memory_ranges: Option<vk::FnInvalidateMappedMemoryRanges>,
    pub fp_get_device_memory_commitment: Option<vk::FnGetDeviceMemoryCommitment>,
    pub fp_get_buffer_memory_requirements: Option<vk::FnGetBufferMemoryRequirements>,
    pub fp_bind_buffer_memory: Option<vk::FnBindBufferMemory>,
    pub fp_get_image_memory_requirements: Option<vk::FnGetImageMemoryRequirements>,
    pub fp_bind_image_memory: Option<vk::FnBindImageMemory>,
    pub fp_get_image_sparse_memory_requirements: Option<vk::FnGetImageSparseMemoryRequirements>,
    pub fp_queue_bind_sparse: Option<vk::FnQueueBindSparse>,
    pub fp_create_fence: Option<vk::FnCreateFence>,
    pub fp_destroy_fence: Option<vk::FnDestroyFence>,
    pub fp_reset_fences: Option<vk::FnResetFences>,
    pub fp_get_fence_status: Option<vk::FnGetFenceStatus>,
    pub fp_wait_for_fences: Option<vk::FnWaitForFences>,
    pub fp_create_semaphore: Option<vk::FnCreateSemaphore>,
    pub fp_destroy_semaphore: Option<vk::FnDestroySemaphore>,
    pub fp_create_event: Option<vk::FnCreateEvent>,
    pub fp_destroy_event: Option<vk::FnDestroyEvent>,
    pub fp_get_event_status: Option<vk::FnGetEventStatus>,
    pub fp_set_event: Option<vk::FnSetEvent>,
    pub fp_reset_event: Option<vk::FnResetEvent>,
    pub fp_create_query_pool: Option<vk::FnCreateQueryPool>,
    pub fp_destroy_query_pool: Option<vk::FnDestroyQueryPool>,
    pub fp_get_query_pool_results: Option<vk::FnGetQueryPoolResults>,
    pub fp_reset_query_pool: Option<vk::FnResetQueryPool>,
    pub fp_create_buffer: Option<vk::FnCreateBuffer>,
    pub fp_destroy_buffer: Option<vk::FnDestroyBuffer>,
    pub fp_create_buffer_view: Option<vk::FnCreateBufferView>,
    pub fp_destroy_buffer_view: Option<vk::FnDestroyBufferView>,
    pub fp_create_image: Option<vk::FnCreateImage>,
    pub fp_destroy_image: Option<vk::FnDestroyImage>,
    pub fp_get_image_subresource_layout: Option<vk::FnGetImageSubresourceLayout>,
    pub fp_create_image_view: Option<vk::FnCreateImageView>,
    pub fp_destroy_image_view: Option<vk::FnDestroyImageView>,
    pub fp_create_shader_module: Option<vk::FnCreateShaderModule>,
    pub fp_destroy_shader_module: Option<vk::FnDestroyShaderModule>,
    pub fp_create_pipeline_cache: Option<vk::FnCreatePipelineCache>,
    pub fp_destroy_pipeline_cache: Option<vk::FnDestroyPipelineCache>,
    pub fp_get_pipeline_cache_data: Option<vk::FnGetPipelineCacheData>,
    pub fp_merge_pipeline_caches: Option<vk::FnMergePipelineCaches>,
    pub fp_create_graphics_pipelines: Option<vk::FnCreateGraphicsPipelines>,
    pub fp_create_compute_pipelines: Option<vk::FnCreateComputePipelines>,
    pub fp_destroy_pipeline: Option<vk::FnDestroyPipeline>,
    pub fp_create_pipeline_layout: Option<vk::FnCreatePipelineLayout>,
    pub fp_destroy_pipeline_layout: Option<vk::FnDestroyPipelineLayout>,
    pub fp_create_sampler: Option<vk::FnCreateSampler>,
    pub fp_destroy_sampler: Option<vk::FnDestroySampler>,
    pub fp_create_descriptor_set_layout: Option<vk::FnCreateDescriptorSetLayout>,
    pub fp_destroy_descriptor_set_layout: Option<vk::FnDestroyDescriptorSetLayout>,
    pub fp_create_descriptor_pool: Option<vk::FnCreateDescriptorPool>,
    pub fp_destroy_descriptor_pool: Option<vk::FnDestroyDescriptorPool>,
    pub fp_reset_descriptor_pool: Option<vk::FnResetDescriptorPool>,
    pub fp_allocate_descriptor_sets: Option<vk::FnAllocateDescriptorSets>,
    pub fp_free_descriptor_sets: Option<vk::FnFreeDescriptorSets>,
    pub fp_update_descriptor_sets: Option<vk::FnUpdateDescriptorSets>,
    pub fp_create_framebuffer: Option<vk::FnCreateFramebuffer>,
    pub fp_destroy_framebuffer: Option<vk::FnDestroyFramebuffer>,
    pub fp_create_render_pass: Option<vk::FnCreateRenderPass>,
    pub fp_destroy_render_pass: Option<vk::FnDestroyRenderPass>,
    pub fp_get_render_area_granularity: Option<vk::FnGetRenderAreaGranularity>,
    pub fp_create_command_pool: Option<vk::FnCreateCommandPool>,
    pub fp_destroy_command_pool: Option<vk::FnDestroyCommandPool>,
    pub fp_reset_command_pool: Option<vk::FnResetCommandPool>,
    pub fp_allocate_command_buffers: Option<vk::FnAllocateCommandBuffers>,
    pub fp_free_command_buffers: Option<vk::FnFreeCommandBuffers>,
    pub fp_begin_command_buffer: Option<vk::FnBeginCommandBuffer>,
    pub fp_end_command_buffer: Option<vk::FnEndCommandBuffer>,
    pub fp_reset_command_buffer: Option<vk::FnResetCommandBuffer>,
    pub fp_cmd_bind_pipeline: Option<vk::FnCmdBindPipeline>,
    pub fp_cmd_set_viewport: Option<vk::FnCmdSetViewport>,
    pub fp_cmd_set_scissor: Option<vk::FnCmdSetScissor>,
    pub fp_cmd_set_line_width: Option<vk::FnCmdSetLineWidth>,
    pub fp_cmd_set_depth_bias: Option<vk::FnCmdSetDepthBias>,
    pub fp_cmd_set_blend_constants: Option<vk::FnCmdSetBlendConstants>,
    pub fp_cmd_set_depth_bounds: Option<vk::FnCmdSetDepthBounds>,
    pub fp_cmd_set_stencil_compare_mask: Option<vk::FnCmdSetStencilCompareMask>,
    pub fp_cmd_set_stencil_write_mask: Option<vk::FnCmdSetStencilWriteMask>,
    pub fp_cmd_set_stencil_reference: Option<vk::FnCmdSetStencilReference>,
    pub fp_cmd_bind_descriptor_sets: Option<vk::FnCmdBindDescriptorSets>,
    pub fp_cmd_bind_index_buffer: Option<vk::FnCmdBindIndexBuffer>,
    pub fp_cmd_bind_vertex_buffers: Option<vk::FnCmdBindVertexBuffers>,
    pub fp_cmd_draw: Option<vk::FnCmdDraw>,
    pub fp_cmd_draw_indexed: Option<vk::FnCmdDrawIndexed>,
    pub fp_cmd_draw_indirect: Option<vk::FnCmdDrawIndirect>,
    pub fp_cmd_draw_indexed_indirect: Option<vk::FnCmdDrawIndexedIndirect>,
    pub fp_cmd_dispatch: Option<vk::FnCmdDispatch>,
    pub fp_cmd_dispatch_indirect: Option<vk::FnCmdDispatchIndirect>,
    pub fp_cmd_copy_buffer: Option<vk::FnCmdCopyBuffer>,
    pub fp_cmd_copy_image: Option<vk::FnCmdCopyImage>,
    pub fp_cmd_blit_image: Option<vk::FnCmdBlitImage>,
    pub fp_cmd_copy_buffer_to_image: Option<vk::FnCmdCopyBufferToImage>,
    pub fp_cmd_copy_image_to_buffer: Option<vk::FnCmdCopyImageToBuffer>,
    pub fp_cmd_update_buffer: Option<vk::FnCmdUpdateBuffer>,
    pub fp_cmd_fill_buffer: Option<vk::FnCmdFillBuffer>,
    pub fp_cmd_clear_color_image: Option<vk::FnCmdClearColorImage>,
    pub fp_cmd_clear_depth_stencil_image: Option<vk::FnCmdClearDepthStencilImage>,
    pub fp_cmd_clear_attachments: Option<vk::FnCmdClearAttachments>,
    pub fp_cmd_resolve_image: Option<vk::FnCmdResolveImage>,
    pub fp_cmd_set_event: Option<vk::FnCmdSetEvent>,
    pub fp_cmd_reset_event: Option<vk::FnCmdResetEvent>,
    pub fp_cmd_wait_events: Option<vk::FnCmdWaitEvents>,
    pub fp_cmd_pipeline_barrier: Option<vk::FnCmdPipelineBarrier>,
    pub fp_cmd_begin_query: Option<vk::FnCmdBeginQuery>,
    pub fp_cmd_end_query: Option<vk::FnCmdEndQuery>,
    pub fp_cmd_begin_conditional_rendering_ext: Option<vk::FnCmdBeginConditionalRenderingEXT>,
    pub fp_cmd_end_conditional_rendering_ext: Option<vk::FnCmdEndConditionalRenderingEXT>,
    pub fp_cmd_reset_query_pool: Option<vk::FnCmdResetQueryPool>,
    pub fp_cmd_write_timestamp: Option<vk::FnCmdWriteTimestamp>,
    pub fp_cmd_copy_query_pool_results: Option<vk::FnCmdCopyQueryPoolResults>,
    pub fp_cmd_push_constants: Option<vk::FnCmdPushConstants>,
    pub fp_cmd_begin_render_pass: Option<vk::FnCmdBeginRenderPass>,
    pub fp_cmd_next_subpass: Option<vk::FnCmdNextSubpass>,
    pub fp_cmd_end_render_pass: Option<vk::FnCmdEndRenderPass>,
    pub fp_cmd_execute_commands: Option<vk::FnCmdExecuteCommands>,
    pub fp_create_shared_swapchains_khr: Option<vk::FnCreateSharedSwapchainsKHR>,
    pub fp_create_swapchain_khr: Option<vk::FnCreateSwapchainKHR>,
    pub fp_destroy_swapchain_khr: Option<vk::FnDestroySwapchainKHR>,
    pub fp_get_swapchain_images_khr: Option<vk::FnGetSwapchainImagesKHR>,
    pub fp_acquire_next_image_khr: Option<vk::FnAcquireNextImageKHR>,
    pub fp_queue_present_khr: Option<vk::FnQueuePresentKHR>,
    pub fp_debug_marker_set_object_name_ext: Option<vk::FnDebugMarkerSetObjectNameEXT>,
    pub fp_debug_marker_set_object_tag_ext: Option<vk::FnDebugMarkerSetObjectTagEXT>,
    pub fp_cmd_debug_marker_begin_ext: Option<vk::FnCmdDebugMarkerBeginEXT>,
    pub fp_cmd_debug_marker_end_ext: Option<vk::FnCmdDebugMarkerEndEXT>,
    pub fp_cmd_debug_marker_insert_ext: Option<vk::FnCmdDebugMarkerInsertEXT>,
    pub fp_get_memory_win32_handle_nv: Option<vk::FnGetMemoryWin32HandleNV>,
    pub fp_cmd_execute_generated_commands_nv: Option<vk::FnCmdExecuteGeneratedCommandsNV>,
    pub fp_cmd_preprocess_generated_commands_nv: Option<vk::FnCmdPreprocessGeneratedCommandsNV>,
    pub fp_cmd_bind_pipeline_shader_group_nv: Option<vk::FnCmdBindPipelineShaderGroupNV>,
    pub fp_get_generated_commands_memory_requirements_nv: Option<vk::FnGetGeneratedCommandsMemoryRequirementsNV>,
    pub fp_create_indirect_commands_layout_nv: Option<vk::FnCreateIndirectCommandsLayoutNV>,
    pub fp_destroy_indirect_commands_layout_nv: Option<vk::FnDestroyIndirectCommandsLayoutNV>,
    pub fp_cmd_push_descriptor_set_khr: Option<vk::FnCmdPushDescriptorSetKHR>,
    pub fp_trim_command_pool: Option<vk::FnTrimCommandPool>,
    pub fp_get_memory_win32_handle_khr: Option<vk::FnGetMemoryWin32HandleKHR>,
    pub fp_get_memory_win32_handle_properties_khr: Option<vk::FnGetMemoryWin32HandlePropertiesKHR>,
    pub fp_get_memory_fd_khr: Option<vk::FnGetMemoryFdKHR>,
    pub fp_get_memory_fd_properties_khr: Option<vk::FnGetMemoryFdPropertiesKHR>,
    pub fp_get_memory_zircon_handle_fuchsia: Option<vk::FnGetMemoryZirconHandleFUCHSIA>,
    pub fp_get_memory_zircon_handle_properties_fuchsia: Option<vk::FnGetMemoryZirconHandlePropertiesFUCHSIA>,
    pub fp_get_semaphore_win32_handle_khr: Option<vk::FnGetSemaphoreWin32HandleKHR>,
    pub fp_import_semaphore_win32_handle_khr: Option<vk::FnImportSemaphoreWin32HandleKHR>,
    pub fp_get_semaphore_fd_khr: Option<vk::FnGetSemaphoreFdKHR>,
    pub fp_import_semaphore_fd_khr: Option<vk::FnImportSemaphoreFdKHR>,
    pub fp_get_semaphore_zircon_handle_fuchsia: Option<vk::FnGetSemaphoreZirconHandleFUCHSIA>,
    pub fp_import_semaphore_zircon_handle_fuchsia: Option<vk::FnImportSemaphoreZirconHandleFUCHSIA>,
    pub fp_get_fence_win32_handle_khr: Option<vk::FnGetFenceWin32HandleKHR>,
    pub fp_import_fence_win32_handle_khr: Option<vk::FnImportFenceWin32HandleKHR>,
    pub fp_get_fence_fd_khr: Option<vk::FnGetFenceFdKHR>,
    pub fp_import_fence_fd_khr: Option<vk::FnImportFenceFdKHR>,
    pub fp_acquire_winrt_display_nv: Option<vk::FnAcquireWinrtDisplayNV>,
    pub fp_get_winrt_display_nv: Option<vk::FnGetWinrtDisplayNV>,
    pub fp_display_power_control_ext: Option<vk::FnDisplayPowerControlEXT>,
    pub fp_register_device_event_ext: Option<vk::FnRegisterDeviceEventEXT>,
    pub fp_register_display_event_ext: Option<vk::FnRegisterDisplayEventEXT>,
    pub fp_get_swapchain_counter_ext: Option<vk::FnGetSwapchainCounterEXT>,
    pub fp_get_device_group_peer_memory_features: Option<vk::FnGetDeviceGroupPeerMemoryFeatures>,
    pub fp_bind_buffer_memory2: Option<vk::FnBindBufferMemory2>,
    pub fp_bind_image_memory2: Option<vk::FnBindImageMemory2>,
    pub fp_cmd_set_device_mask: Option<vk::FnCmdSetDeviceMask>,
    pub fp_get_device_group_present_capabilities_khr: Option<vk::FnGetDeviceGroupPresentCapabilitiesKHR>,
    pub fp_get_device_group_surface_present_modes_khr: Option<vk::FnGetDeviceGroupSurfacePresentModesKHR>,
    pub fp_acquire_next_image2_khr: Option<vk::FnAcquireNextImage2KHR>,
    pub fp_cmd_dispatch_base: Option<vk::FnCmdDispatchBase>,
    pub fp_get_physical_device_present_rectangles_khr: Option<vk::FnGetPhysicalDevicePresentRectanglesKHR>,
    pub fp_create_descriptor_update_template: Option<vk::FnCreateDescriptorUpdateTemplate>,
    pub fp_destroy_descriptor_update_template: Option<vk::FnDestroyDescriptorUpdateTemplate>,
    pub fp_update_descriptor_set_with_template: Option<vk::FnUpdateDescriptorSetWithTemplate>,
    pub fp_cmd_push_descriptor_set_with_template_khr: Option<vk::FnCmdPushDescriptorSetWithTemplateKHR>,
    pub fp_set_hdr_metadata_ext: Option<vk::FnSetHdrMetadataEXT>,
    pub fp_get_swapchain_status_khr: Option<vk::FnGetSwapchainStatusKHR>,
    pub fp_get_refresh_cycle_duration_google: Option<vk::FnGetRefreshCycleDurationGOOGLE>,
    pub fp_get_past_presentation_timing_google: Option<vk::FnGetPastPresentationTimingGOOGLE>,
    pub fp_cmd_set_viewport_w_scaling_nv: Option<vk::FnCmdSetViewportWScalingNV>,
    pub fp_cmd_set_discard_rectangle_ext: Option<vk::FnCmdSetDiscardRectangleEXT>,
    pub fp_cmd_set_sample_locations_ext: Option<vk::FnCmdSetSampleLocationsEXT>,
    pub fp_get_physical_device_multisample_properties_ext: Option<vk::FnGetPhysicalDeviceMultisamplePropertiesEXT>,
    pub fp_get_buffer_memory_requirements2: Option<vk::FnGetBufferMemoryRequirements2>,
    pub fp_get_image_memory_requirements2: Option<vk::FnGetImageMemoryRequirements2>,
    pub fp_get_image_sparse_memory_requirements2: Option<vk::FnGetImageSparseMemoryRequirements2>,
    pub fp_create_sampler_ycbcr_conversion: Option<vk::FnCreateSamplerYcbcrConversion>,
    pub fp_destroy_sampler_ycbcr_conversion: Option<vk::FnDestroySamplerYcbcrConversion>,
    pub fp_get_device_queue2: Option<vk::FnGetDeviceQueue2>,
    pub fp_create_validation_cache_ext: Option<vk::FnCreateValidationCacheEXT>,
    pub fp_destroy_validation_cache_ext: Option<vk::FnDestroyValidationCacheEXT>,
    pub fp_get_validation_cache_data_ext: Option<vk::FnGetValidationCacheDataEXT>,
    pub fp_merge_validation_caches_ext: Option<vk::FnMergeValidationCachesEXT>,
    pub fp_get_descriptor_set_layout_support: Option<vk::FnGetDescriptorSetLayoutSupport>,
    pub fp_get_shader_info_amd: Option<vk::FnGetShaderInfoAMD>,
    pub fp_set_local_dimming_amd: Option<vk::FnSetLocalDimmingAMD>,
    pub fp_get_physical_device_calibrateable_time_domains_ext:
        Option<vk::FnGetPhysicalDeviceCalibrateableTimeDomainsEXT>,
    pub fp_get_calibrated_timestamps_ext: Option<vk::FnGetCalibratedTimestampsEXT>,
    pub fp_get_memory_host_pointer_properties_ext: Option<vk::FnGetMemoryHostPointerPropertiesEXT>,
    pub fp_cmd_write_buffer_marker_amd: Option<vk::FnCmdWriteBufferMarkerAMD>,
    pub fp_create_render_pass2: Option<vk::FnCreateRenderPass2>,
    pub fp_cmd_begin_render_pass2: Option<vk::FnCmdBeginRenderPass2>,
    pub fp_cmd_next_subpass2: Option<vk::FnCmdNextSubpass2>,
    pub fp_cmd_end_render_pass2: Option<vk::FnCmdEndRenderPass2>,
    pub fp_get_semaphore_counter_value: Option<vk::FnGetSemaphoreCounterValue>,
    pub fp_wait_semaphores: Option<vk::FnWaitSemaphores>,
    pub fp_signal_semaphore: Option<vk::FnSignalSemaphore>,
    pub fp_get_android_hardware_buffer_properties_android: Option<vk::FnGetAndroidHardwareBufferPropertiesANDROID>,
    pub fp_get_memory_android_hardware_buffer_android: Option<vk::FnGetMemoryAndroidHardwareBufferANDROID>,
    pub fp_cmd_draw_indirect_count: Option<vk::FnCmdDrawIndirectCount>,
    pub fp_cmd_draw_indexed_indirect_count: Option<vk::FnCmdDrawIndexedIndirectCount>,
    pub fp_cmd_set_checkpoint_nv: Option<vk::FnCmdSetCheckpointNV>,
    pub fp_get_queue_checkpoint_data_nv: Option<vk::FnGetQueueCheckpointDataNV>,
    pub fp_cmd_bind_transform_feedback_buffers_ext: Option<vk::FnCmdBindTransformFeedbackBuffersEXT>,
    pub fp_cmd_begin_transform_feedback_ext: Option<vk::FnCmdBeginTransformFeedbackEXT>,
    pub fp_cmd_end_transform_feedback_ext: Option<vk::FnCmdEndTransformFeedbackEXT>,
    pub fp_cmd_begin_query_indexed_ext: Option<vk::FnCmdBeginQueryIndexedEXT>,
    pub fp_cmd_end_query_indexed_ext: Option<vk::FnCmdEndQueryIndexedEXT>,
    pub fp_cmd_draw_indirect_byte_count_ext: Option<vk::FnCmdDrawIndirectByteCountEXT>,
    pub fp_cmd_set_exclusive_scissor_nv: Option<vk::FnCmdSetExclusiveScissorNV>,
    pub fp_cmd_bind_shading_rate_image_nv: Option<vk::FnCmdBindShadingRateImageNV>,
    pub fp_cmd_set_viewport_shading_rate_palette_nv: Option<vk::FnCmdSetViewportShadingRatePaletteNV>,
    pub fp_cmd_set_coarse_sample_order_nv: Option<vk::FnCmdSetCoarseSampleOrderNV>,
    pub fp_cmd_draw_mesh_tasks_nv: Option<vk::FnCmdDrawMeshTasksNV>,
    pub fp_cmd_draw_mesh_tasks_indirect_nv: Option<vk::FnCmdDrawMeshTasksIndirectNV>,
    pub fp_cmd_draw_mesh_tasks_indirect_count_nv: Option<vk::FnCmdDrawMeshTasksIndirectCountNV>,
    pub fp_compile_deferred_nv: Option<vk::FnCompileDeferredNV>,
    pub fp_create_acceleration_structure_nv: Option<vk::FnCreateAccelerationStructureNV>,
    pub fp_destroy_acceleration_structure_khr: Option<vk::FnDestroyAccelerationStructureKHR>,
    pub fp_destroy_acceleration_structure_nv: Option<vk::FnDestroyAccelerationStructureNV>,
    pub fp_get_acceleration_structure_memory_requirements_nv:
        Option<vk::FnGetAccelerationStructureMemoryRequirementsNV>,
    pub fp_bind_acceleration_structure_memory_nv: Option<vk::FnBindAccelerationStructureMemoryNV>,
    pub fp_cmd_copy_acceleration_structure_nv: Option<vk::FnCmdCopyAccelerationStructureNV>,
    pub fp_cmd_copy_acceleration_structure_khr: Option<vk::FnCmdCopyAccelerationStructureKHR>,
    pub fp_copy_acceleration_structure_khr: Option<vk::FnCopyAccelerationStructureKHR>,
    pub fp_cmd_copy_acceleration_structure_to_memory_khr: Option<vk::FnCmdCopyAccelerationStructureToMemoryKHR>,
    pub fp_copy_acceleration_structure_to_memory_khr: Option<vk::FnCopyAccelerationStructureToMemoryKHR>,
    pub fp_cmd_copy_memory_to_acceleration_structure_khr: Option<vk::FnCmdCopyMemoryToAccelerationStructureKHR>,
    pub fp_copy_memory_to_acceleration_structure_khr: Option<vk::FnCopyMemoryToAccelerationStructureKHR>,
    pub fp_cmd_write_acceleration_structures_properties_khr: Option<vk::FnCmdWriteAccelerationStructuresPropertiesKHR>,
    pub fp_cmd_write_acceleration_structures_properties_nv: Option<vk::FnCmdWriteAccelerationStructuresPropertiesNV>,
    pub fp_cmd_build_acceleration_structure_nv: Option<vk::FnCmdBuildAccelerationStructureNV>,
    pub fp_write_acceleration_structures_properties_khr: Option<vk::FnWriteAccelerationStructuresPropertiesKHR>,
    pub fp_cmd_trace_rays_khr: Option<vk::FnCmdTraceRaysKHR>,
    pub fp_cmd_trace_rays_nv: Option<vk::FnCmdTraceRaysNV>,
    pub fp_get_ray_tracing_shader_group_handles_khr: Option<vk::FnGetRayTracingShaderGroupHandlesKHR>,
    pub fp_get_ray_tracing_capture_replay_shader_group_handles_khr:
        Option<vk::FnGetRayTracingCaptureReplayShaderGroupHandlesKHR>,
    pub fp_get_acceleration_structure_handle_nv: Option<vk::FnGetAccelerationStructureHandleNV>,
    pub fp_create_ray_tracing_pipelines_nv: Option<vk::FnCreateRayTracingPipelinesNV>,
    pub fp_create_ray_tracing_pipelines_khr: Option<vk::FnCreateRayTracingPipelinesKHR>,
    pub fp_get_physical_device_cooperative_matrix_properties_nv:
        Option<vk::FnGetPhysicalDeviceCooperativeMatrixPropertiesNV>,
    pub fp_cmd_trace_rays_indirect_khr: Option<vk::FnCmdTraceRaysIndirectKHR>,
    pub fp_get_device_acceleration_structure_compatibility_khr:
        Option<vk::FnGetDeviceAccelerationStructureCompatibilityKHR>,
    pub fp_get_ray_tracing_shader_group_stack_size_khr: Option<vk::FnGetRayTracingShaderGroupStackSizeKHR>,
    pub fp_cmd_set_ray_tracing_pipeline_stack_size_khr: Option<vk::FnCmdSetRayTracingPipelineStackSizeKHR>,
    pub fp_get_image_view_handle_nvx: Option<vk::FnGetImageViewHandleNVX>,
    pub fp_get_image_view_address_nvx: Option<vk::FnGetImageViewAddressNVX>,
    pub fp_get_physical_device_surface_present_modes2_ext: Option<vk::FnGetPhysicalDeviceSurfacePresentModes2EXT>,
    pub fp_get_device_group_surface_present_modes2_ext: Option<vk::FnGetDeviceGroupSurfacePresentModes2EXT>,
    pub fp_acquire_full_screen_exclusive_mode_ext: Option<vk::FnAcquireFullScreenExclusiveModeEXT>,
    pub fp_release_full_screen_exclusive_mode_ext: Option<vk::FnReleaseFullScreenExclusiveModeEXT>,
    pub fp_enumerate_physical_device_queue_family_performance_query_counters_khr:
        Option<vk::FnEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR>,
    pub fp_get_physical_device_queue_family_performance_query_passes_khr:
        Option<vk::FnGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR>,
    pub fp_acquire_profiling_lock_khr: Option<vk::FnAcquireProfilingLockKHR>,
    pub fp_release_profiling_lock_khr: Option<vk::FnReleaseProfilingLockKHR>,
    pub fp_get_image_drm_format_modifier_properties_ext: Option<vk::FnGetImageDrmFormatModifierPropertiesEXT>,
    pub fp_get_buffer_opaque_capture_address: Option<vk::FnGetBufferOpaqueCaptureAddress>,
    pub fp_get_buffer_device_address: Option<vk::FnGetBufferDeviceAddress>,
    pub fp_get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        Option<vk::FnGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV>,
    pub fp_initialize_performance_api_intel: Option<vk::FnInitializePerformanceApiINTEL>,
    pub fp_uninitialize_performance_api_intel: Option<vk::FnUninitializePerformanceApiINTEL>,
    pub fp_cmd_set_performance_marker_intel: Option<vk::FnCmdSetPerformanceMarkerINTEL>,
    pub fp_cmd_set_performance_stream_marker_intel: Option<vk::FnCmdSetPerformanceStreamMarkerINTEL>,
    pub fp_cmd_set_performance_override_intel: Option<vk::FnCmdSetPerformanceOverrideINTEL>,
    pub fp_acquire_performance_configuration_intel: Option<vk::FnAcquirePerformanceConfigurationINTEL>,
    pub fp_release_performance_configuration_intel: Option<vk::FnReleasePerformanceConfigurationINTEL>,
    pub fp_queue_set_performance_configuration_intel: Option<vk::FnQueueSetPerformanceConfigurationINTEL>,
    pub fp_get_performance_parameter_intel: Option<vk::FnGetPerformanceParameterINTEL>,
    pub fp_get_device_memory_opaque_capture_address: Option<vk::FnGetDeviceMemoryOpaqueCaptureAddress>,
    pub fp_get_pipeline_executable_properties_khr: Option<vk::FnGetPipelineExecutablePropertiesKHR>,
    pub fp_get_pipeline_executable_statistics_khr: Option<vk::FnGetPipelineExecutableStatisticsKHR>,
    pub fp_get_pipeline_executable_internal_representations_khr:
        Option<vk::FnGetPipelineExecutableInternalRepresentationsKHR>,
    pub fp_cmd_set_line_stipple_ext: Option<vk::FnCmdSetLineStippleEXT>,
    pub fp_get_physical_device_tool_properties_ext: Option<vk::FnGetPhysicalDeviceToolPropertiesEXT>,
    pub fp_create_acceleration_structure_khr: Option<vk::FnCreateAccelerationStructureKHR>,
    pub fp_cmd_build_acceleration_structures_khr: Option<vk::FnCmdBuildAccelerationStructuresKHR>,
    pub fp_cmd_build_acceleration_structures_indirect_khr: Option<vk::FnCmdBuildAccelerationStructuresIndirectKHR>,
    pub fp_build_acceleration_structures_khr: Option<vk::FnBuildAccelerationStructuresKHR>,
    pub fp_get_acceleration_structure_device_address_khr: Option<vk::FnGetAccelerationStructureDeviceAddressKHR>,
    pub fp_create_deferred_operation_khr: Option<vk::FnCreateDeferredOperationKHR>,
    pub fp_destroy_deferred_operation_khr: Option<vk::FnDestroyDeferredOperationKHR>,
    pub fp_get_deferred_operation_max_concurrency_khr: Option<vk::FnGetDeferredOperationMaxConcurrencyKHR>,
    pub fp_get_deferred_operation_result_khr: Option<vk::FnGetDeferredOperationResultKHR>,
    pub fp_deferred_operation_join_khr: Option<vk::FnDeferredOperationJoinKHR>,
    pub fp_cmd_set_cull_mode_ext: Option<vk::FnCmdSetCullModeEXT>,
    pub fp_cmd_set_front_face_ext: Option<vk::FnCmdSetFrontFaceEXT>,
    pub fp_cmd_set_primitive_topology_ext: Option<vk::FnCmdSetPrimitiveTopologyEXT>,
    pub fp_cmd_set_viewport_with_count_ext: Option<vk::FnCmdSetViewportWithCountEXT>,
    pub fp_cmd_set_scissor_with_count_ext: Option<vk::FnCmdSetScissorWithCountEXT>,
    pub fp_cmd_bind_vertex_buffers2_ext: Option<vk::FnCmdBindVertexBuffers2EXT>,
    pub fp_cmd_set_depth_test_enable_ext: Option<vk::FnCmdSetDepthTestEnableEXT>,
    pub fp_cmd_set_depth_write_enable_ext: Option<vk::FnCmdSetDepthWriteEnableEXT>,
    pub fp_cmd_set_depth_compare_op_ext: Option<vk::FnCmdSetDepthCompareOpEXT>,
    pub fp_cmd_set_depth_bounds_test_enable_ext: Option<vk::FnCmdSetDepthBoundsTestEnableEXT>,
    pub fp_cmd_set_stencil_test_enable_ext: Option<vk::FnCmdSetStencilTestEnableEXT>,
    pub fp_cmd_set_stencil_op_ext: Option<vk::FnCmdSetStencilOpEXT>,
    pub fp_create_private_data_slot_ext: Option<vk::FnCreatePrivateDataSlotEXT>,
    pub fp_destroy_private_data_slot_ext: Option<vk::FnDestroyPrivateDataSlotEXT>,
    pub fp_set_private_data_ext: Option<vk::FnSetPrivateDataEXT>,
    pub fp_get_private_data_ext: Option<vk::FnGetPrivateDataEXT>,
    pub fp_cmd_copy_buffer2_khr: Option<vk::FnCmdCopyBuffer2KHR>,
    pub fp_cmd_copy_image2_khr: Option<vk::FnCmdCopyImage2KHR>,
    pub fp_cmd_blit_image2_khr: Option<vk::FnCmdBlitImage2KHR>,
    pub fp_cmd_copy_buffer_to_image2_khr: Option<vk::FnCmdCopyBufferToImage2KHR>,
    pub fp_cmd_copy_image_to_buffer2_khr: Option<vk::FnCmdCopyImageToBuffer2KHR>,
    pub fp_cmd_resolve_image2_khr: Option<vk::FnCmdResolveImage2KHR>,
    pub fp_cmd_set_fragment_shading_rate_khr: Option<vk::FnCmdSetFragmentShadingRateKHR>,
    pub fp_get_physical_device_fragment_shading_rates_khr: Option<vk::FnGetPhysicalDeviceFragmentShadingRatesKHR>,
    pub fp_cmd_set_fragment_shading_rate_enum_nv: Option<vk::FnCmdSetFragmentShadingRateEnumNV>,
    pub fp_get_acceleration_structure_build_sizes_khr: Option<vk::FnGetAccelerationStructureBuildSizesKHR>,
    pub fp_cmd_set_vertex_input_ext: Option<vk::FnCmdSetVertexInputEXT>,
    pub fp_cmd_set_color_write_enable_ext: Option<vk::FnCmdSetColorWriteEnableEXT>,
    pub fp_cmd_set_event2_khr: Option<vk::FnCmdSetEvent2KHR>,
    pub fp_cmd_reset_event2_khr: Option<vk::FnCmdResetEvent2KHR>,
    pub fp_cmd_wait_events2_khr: Option<vk::FnCmdWaitEvents2KHR>,
    pub fp_cmd_pipeline_barrier2_khr: Option<vk::FnCmdPipelineBarrier2KHR>,
    pub fp_queue_submit2_khr: Option<vk::FnQueueSubmit2KHR>,
    pub fp_cmd_write_timestamp2_khr: Option<vk::FnCmdWriteTimestamp2KHR>,
    pub fp_cmd_write_buffer_marker2_amd: Option<vk::FnCmdWriteBufferMarker2AMD>,
    pub fp_get_queue_checkpoint_data2_nv: Option<vk::FnGetQueueCheckpointData2NV>,
}
impl Device {
    #[allow(clippy::cognitive_complexity, clippy::nonminimal_bool)]
    pub unsafe fn load(
        instance: &Instance,
        device: vk::Device,
        create_info: &vk::DeviceCreateInfo,
        version: vk::Version,
    ) -> LoaderResult<Self> {
        let mut extensions = DeviceExtensions::new(version);
        if create_info.enabled_extension_count != 0 {
            for &name_ptr in slice::from_raw_parts(
                create_info.pp_enabled_extension_names,
                create_info.enabled_extension_count as usize,
            ) {
                extensions.enable_by_name(&CStr::from_ptr(name_ptr));
            }
        }
        let f = |name: &CStr| instance.get_device_proc_addr(device, name);
        Ok(Self {
            handle: device,
            extensions,
            fp_destroy_device: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyDevice\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyDevice".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_device_queue: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetDeviceQueue".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_queue_submit: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkQueueSubmit".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_queue_wait_idle: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkQueueWaitIdle\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkQueueWaitIdle".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_device_wait_idle: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDeviceWaitIdle\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDeviceWaitIdle".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_allocate_memory: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkAllocateMemory\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkAllocateMemory".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_free_memory: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkFreeMemory\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkFreeMemory".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_map_memory: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkMapMemory\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkMapMemory".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_unmap_memory: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkUnmapMemory\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkUnmapMemory".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_flush_mapped_memory_ranges: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkFlushMappedMemoryRanges\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkFlushMappedMemoryRanges".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_invalidate_mapped_memory_ranges: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkInvalidateMappedMemoryRanges\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkInvalidateMappedMemoryRanges".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_device_memory_commitment: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceMemoryCommitment\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetDeviceMemoryCommitment".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_buffer_memory_requirements: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetBufferMemoryRequirements\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetBufferMemoryRequirements".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_bind_buffer_memory: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkBindBufferMemory".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_image_memory_requirements: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetImageMemoryRequirements\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetImageMemoryRequirements".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_bind_image_memory: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkBindImageMemory".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_image_sparse_memory_requirements: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSparseMemoryRequirements\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetImageSparseMemoryRequirements".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_queue_bind_sparse: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkQueueBindSparse\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkQueueBindSparse".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_fence: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateFence\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateFence".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_fence: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyFence\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyFence".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_reset_fences: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkResetFences\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkResetFences".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_fence_status: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetFenceStatus\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetFenceStatus".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_wait_for_fences: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkWaitForFences\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkWaitForFences".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_semaphore: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSemaphore\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateSemaphore".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_semaphore: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroySemaphore\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroySemaphore".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_event: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateEvent\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateEvent".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_event: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyEvent\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyEvent".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_event_status: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetEventStatus\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetEventStatus".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_set_event: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSetEvent\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkSetEvent".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_reset_event: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkResetEvent\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkResetEvent".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_query_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateQueryPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateQueryPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_query_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyQueryPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyQueryPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_query_pool_results: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetQueryPoolResults\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetQueryPoolResults".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_reset_query_pool: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkResetQueryPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkResetQueryPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.ext_host_query_reset {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkResetQueryPoolEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_buffer_view: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateBufferView\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateBufferView".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_buffer_view: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyBufferView\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyBufferView".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_image: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateImage\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateImage".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_image: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyImage\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyImage".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_image_subresource_layout: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetImageSubresourceLayout\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetImageSubresourceLayout".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_image_view: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateImageView\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateImageView".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_image_view: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyImageView\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyImageView".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_shader_module: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateShaderModule\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateShaderModule".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_shader_module: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyShaderModule\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyShaderModule".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_pipeline_cache: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreatePipelineCache\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreatePipelineCache".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_pipeline_cache: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipelineCache\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyPipelineCache".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_pipeline_cache_data: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetPipelineCacheData\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetPipelineCacheData".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_merge_pipeline_caches: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkMergePipelineCaches\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkMergePipelineCaches".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_graphics_pipelines: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateGraphicsPipelines\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateGraphicsPipelines".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_compute_pipelines: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateComputePipelines\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateComputePipelines".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_pipeline: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipeline\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyPipeline".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_pipeline_layout: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreatePipelineLayout\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreatePipelineLayout".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_pipeline_layout: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipelineLayout\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyPipelineLayout".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_sampler: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSampler\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateSampler".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_sampler: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroySampler\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroySampler".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_descriptor_set_layout: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDescriptorSetLayout\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateDescriptorSetLayout".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_descriptor_set_layout: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyDescriptorSetLayout\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyDescriptorSetLayout".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_descriptor_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDescriptorPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateDescriptorPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_descriptor_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyDescriptorPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyDescriptorPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_reset_descriptor_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkResetDescriptorPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkResetDescriptorPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_allocate_descriptor_sets: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkAllocateDescriptorSets\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkAllocateDescriptorSets".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_free_descriptor_sets: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkFreeDescriptorSets\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkFreeDescriptorSets".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_update_descriptor_sets: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkUpdateDescriptorSets\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkUpdateDescriptorSets".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_framebuffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateFramebuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateFramebuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_framebuffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyFramebuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyFramebuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_render_pass: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateRenderPass".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_render_pass: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyRenderPass\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyRenderPass".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_get_render_area_granularity: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetRenderAreaGranularity\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetRenderAreaGranularity".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_command_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateCommandPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateCommandPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_destroy_command_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyCommandPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkDestroyCommandPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_reset_command_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkResetCommandPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkResetCommandPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_allocate_command_buffers: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkAllocateCommandBuffers\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkAllocateCommandBuffers".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_free_command_buffers: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkFreeCommandBuffers\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkFreeCommandBuffers".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_begin_command_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkBeginCommandBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkBeginCommandBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_end_command_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkEndCommandBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkEndCommandBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_reset_command_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkResetCommandBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkResetCommandBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_bind_pipeline: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindPipeline\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdBindPipeline".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_viewport: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewport\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetViewport".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_scissor: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissor\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetScissor".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_line_width: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineWidth\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetLineWidth".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_depth_bias: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBias\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetDepthBias".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_blend_constants: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetBlendConstants\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetBlendConstants".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_depth_bounds: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBounds\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetDepthBounds".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_stencil_compare_mask: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilCompareMask\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetStencilCompareMask".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_stencil_write_mask: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilWriteMask\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetStencilWriteMask".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_stencil_reference: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilReference\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetStencilReference".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_bind_descriptor_sets: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindDescriptorSets\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdBindDescriptorSets".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_bind_index_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindIndexBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdBindIndexBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_bind_vertex_buffers: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdBindVertexBuffers".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_draw: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDraw\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDraw".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_draw_indexed: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexed\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDrawIndexed".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_draw_indirect: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirect\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDrawIndirect".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_draw_indexed_indirect: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexedIndirect\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDrawIndexedIndirect".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_dispatch: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatch\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDispatch".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_dispatch_indirect: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchIndirect\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDispatchIndirect".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_copy_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdCopyBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_copy_image: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdCopyImage".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_blit_image: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdBlitImage".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_copy_buffer_to_image: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBufferToImage\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdCopyBufferToImage".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_copy_image_to_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImageToBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdCopyImageToBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_update_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdUpdateBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdUpdateBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_fill_buffer: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdFillBuffer\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdFillBuffer".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_clear_color_image: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdClearColorImage\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdClearColorImage".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_clear_depth_stencil_image: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdClearDepthStencilImage\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdClearDepthStencilImage".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_clear_attachments: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdClearAttachments\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdClearAttachments".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_resolve_image: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdResolveImage".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_set_event: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetEvent".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_reset_event: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdResetEvent".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_wait_events: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdWaitEvents".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_pipeline_barrier: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdPipelineBarrier".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_begin_query: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQuery\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdBeginQuery".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_end_query: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQuery\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdEndQuery".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_begin_conditional_rendering_ext: if extensions.ext_conditional_rendering {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBeginConditionalRenderingEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_end_conditional_rendering_ext: if extensions.ext_conditional_rendering {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdEndConditionalRenderingEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_reset_query_pool: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdResetQueryPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdResetQueryPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_write_timestamp: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdWriteTimestamp".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_copy_query_pool_results: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyQueryPoolResults\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdCopyQueryPoolResults".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_push_constants: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdPushConstants\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdPushConstants".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_begin_render_pass: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdBeginRenderPass".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_next_subpass: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdNextSubpass".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_end_render_pass: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdEndRenderPass".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_cmd_execute_commands: {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdExecuteCommands\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdExecuteCommands".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            },
            fp_create_shared_swapchains_khr: if extensions.khr_display_swapchain {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSharedSwapchainsKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_swapchain_khr: if extensions.khr_swapchain {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSwapchainKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_swapchain_khr: if extensions.khr_swapchain {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroySwapchainKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_swapchain_images_khr: if extensions.khr_swapchain {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainImagesKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_acquire_next_image_khr: if extensions.khr_swapchain {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImageKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_queue_present_khr: if extensions.khr_swapchain {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkQueuePresentKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_debug_marker_set_object_name_ext: if extensions.ext_debug_marker {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDebugMarkerSetObjectNameEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_debug_marker_set_object_tag_ext: if extensions.ext_debug_marker {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDebugMarkerSetObjectTagEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_debug_marker_begin_ext: if extensions.ext_debug_marker {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerBeginEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_debug_marker_end_ext: if extensions.ext_debug_marker {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerEndEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_debug_marker_insert_ext: if extensions.ext_debug_marker {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerInsertEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_win32_handle_nv: if extensions.nv_external_memory_win32 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_execute_generated_commands_nv: if extensions.nv_device_generated_commands {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdExecuteGeneratedCommandsNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_preprocess_generated_commands_nv: if extensions.nv_device_generated_commands {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPreprocessGeneratedCommandsNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_bind_pipeline_shader_group_nv: if extensions.nv_device_generated_commands {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindPipelineShaderGroupNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_generated_commands_memory_requirements_nv: if extensions.nv_device_generated_commands {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetGeneratedCommandsMemoryRequirementsNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_indirect_commands_layout_nv: if extensions.nv_device_generated_commands {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateIndirectCommandsLayoutNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_indirect_commands_layout_nv: if extensions.nv_device_generated_commands {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyIndirectCommandsLayoutNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_push_descriptor_set_khr: if extensions.khr_push_descriptor {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdPushDescriptorSetKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_trim_command_pool: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPool\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkTrimCommandPool".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_maintenance1 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPoolKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_win32_handle_khr: if extensions.khr_external_memory_win32 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_win32_handle_properties_khr: if extensions.khr_external_memory_win32 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryWin32HandlePropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_fd_khr: if extensions.khr_external_memory_fd {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryFdKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_fd_properties_khr: if extensions.khr_external_memory_fd {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryFdPropertiesKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_zircon_handle_fuchsia: if extensions.fuchsia_external_memory {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryZirconHandleFUCHSIA\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_zircon_handle_properties_fuchsia: if extensions.fuchsia_external_memory {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryZirconHandlePropertiesFUCHSIA\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_semaphore_win32_handle_khr: if extensions.khr_external_semaphore_win32 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreWin32HandleKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_import_semaphore_win32_handle_khr: if extensions.khr_external_semaphore_win32 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkImportSemaphoreWin32HandleKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_semaphore_fd_khr: if extensions.khr_external_semaphore_fd {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreFdKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_import_semaphore_fd_khr: if extensions.khr_external_semaphore_fd {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkImportSemaphoreFdKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_semaphore_zircon_handle_fuchsia: if extensions.fuchsia_external_semaphore {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSemaphoreZirconHandleFUCHSIA\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_import_semaphore_zircon_handle_fuchsia: if extensions.fuchsia_external_semaphore {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkImportSemaphoreZirconHandleFUCHSIA\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_fence_win32_handle_khr: if extensions.khr_external_fence_win32 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetFenceWin32HandleKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_import_fence_win32_handle_khr: if extensions.khr_external_fence_win32 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkImportFenceWin32HandleKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_fence_fd_khr: if extensions.khr_external_fence_fd {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetFenceFdKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_import_fence_fd_khr: if extensions.khr_external_fence_fd {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkImportFenceFdKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_acquire_winrt_display_nv: if extensions.nv_acquire_winrt_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkAcquireWinrtDisplayNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_winrt_display_nv: if extensions.nv_acquire_winrt_display {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetWinrtDisplayNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_display_power_control_ext: if extensions.ext_display_control {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDisplayPowerControlEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_register_device_event_ext: if extensions.ext_display_control {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkRegisterDeviceEventEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_register_display_event_ext: if extensions.ext_display_control {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkRegisterDisplayEventEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_swapchain_counter_ext: if extensions.ext_display_control {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainCounterEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_device_group_peer_memory_features: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPeerMemoryFeatures\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetDeviceGroupPeerMemoryFeatures".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_device_group {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPeerMemoryFeaturesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_bind_buffer_memory2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkBindBufferMemory2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_bind_memory2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_bind_image_memory2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkBindImageMemory2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_bind_memory2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_device_mask: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMask\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdSetDeviceMask".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_device_group {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMaskKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_device_group_present_capabilities_khr: if (extensions.khr_swapchain
                && version >= vk::Version::from_raw_parts(1, 1, 0))
                || (extensions.khr_device_group && instance.extensions.khr_surface)
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_device_group_surface_present_modes_khr: if (extensions.khr_swapchain
                && version >= vk::Version::from_raw_parts(1, 1, 0))
                || (extensions.khr_device_group && instance.extensions.khr_surface)
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupSurfacePresentModesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_acquire_next_image2_khr: if (extensions.khr_swapchain && version >= vk::Version::from_raw_parts(1, 1, 0))
                || (extensions.khr_device_group && extensions.khr_swapchain)
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_dispatch_base: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBase\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDispatchBase".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_device_group {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBaseKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_present_rectangles_khr: if (extensions.khr_swapchain
                && version >= vk::Version::from_raw_parts(1, 1, 0))
                || (extensions.khr_device_group && instance.extensions.khr_surface)
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDevicePresentRectanglesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_descriptor_update_template: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDescriptorUpdateTemplate\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkCreateDescriptorUpdateTemplate".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_descriptor_update_template {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDescriptorUpdateTemplateKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_descriptor_update_template: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDescriptorUpdateTemplate\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkDestroyDescriptorUpdateTemplate".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_descriptor_update_template {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDescriptorUpdateTemplateKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_update_descriptor_set_with_template: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkUpdateDescriptorSetWithTemplate\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkUpdateDescriptorSetWithTemplate".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_descriptor_update_template {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkUpdateDescriptorSetWithTemplateKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_push_descriptor_set_with_template_khr: if (extensions.khr_push_descriptor
                && version >= vk::Version::from_raw_parts(1, 1, 0))
                || (extensions.khr_push_descriptor && extensions.khr_descriptor_update_template)
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPushDescriptorSetWithTemplateKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_set_hdr_metadata_ext: if extensions.ext_hdr_metadata {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSetHdrMetadataEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_swapchain_status_khr: if extensions.khr_shared_presentable_image {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainStatusKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_refresh_cycle_duration_google: if extensions.google_display_timing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRefreshCycleDurationGOOGLE\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_past_presentation_timing_google: if extensions.google_display_timing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPastPresentationTimingGOOGLE\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_viewport_w_scaling_nv: if extensions.nv_clip_space_w_scaling {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWScalingNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_discard_rectangle_ext: if extensions.ext_discard_rectangles {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDiscardRectangleEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_sample_locations_ext: if extensions.ext_sample_locations {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleLocationsEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_multisample_properties_ext: if extensions.ext_sample_locations {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMultisamplePropertiesEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_buffer_memory_requirements2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetBufferMemoryRequirements2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetBufferMemoryRequirements2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_memory_requirements2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferMemoryRequirements2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_image_memory_requirements2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetImageMemoryRequirements2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetImageMemoryRequirements2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_memory_requirements2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageMemoryRequirements2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_image_sparse_memory_requirements2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSparseMemoryRequirements2\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetImageSparseMemoryRequirements2".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_get_memory_requirements2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSparseMemoryRequirements2KHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_sampler_ycbcr_conversion: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSamplerYcbcrConversion\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateSamplerYcbcrConversion".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_sampler_ycbcr_conversion {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateSamplerYcbcrConversionKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_sampler_ycbcr_conversion: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroySamplerYcbcrConversion\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkDestroySamplerYcbcrConversion".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_sampler_ycbcr_conversion {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroySamplerYcbcrConversionKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_device_queue2: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetDeviceQueue2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_validation_cache_ext: if extensions.ext_validation_cache {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateValidationCacheEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_validation_cache_ext: if extensions.ext_validation_cache {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyValidationCacheEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_validation_cache_data_ext: if extensions.ext_validation_cache {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetValidationCacheDataEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_merge_validation_caches_ext: if extensions.ext_validation_cache {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkMergeValidationCachesEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_descriptor_set_layout_support: if version >= vk::Version::from_raw_parts(1, 1, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutSupport\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetDescriptorSetLayoutSupport".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_maintenance3 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutSupportKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_shader_info_amd: if extensions.amd_shader_info {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetShaderInfoAMD\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_set_local_dimming_amd: if extensions.amd_display_native_hdr {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSetLocalDimmingAMD\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_calibrateable_time_domains_ext: if extensions.ext_calibrated_timestamps {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_calibrated_timestamps_ext: if extensions.ext_calibrated_timestamps {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetCalibratedTimestampsEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_host_pointer_properties_ext: if extensions.ext_external_memory_host {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryHostPointerPropertiesEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_write_buffer_marker_amd: if extensions.amd_buffer_marker {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteBufferMarkerAMD\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_render_pass2: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCreateRenderPass2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_create_renderpass2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_begin_render_pass2: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdBeginRenderPass2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_create_renderpass2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_next_subpass2: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdNextSubpass2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_create_renderpass2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_end_render_pass2: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass2\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdEndRenderPass2".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_create_renderpass2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_semaphore_counter_value: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreCounterValue\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetSemaphoreCounterValue".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_timeline_semaphore {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreCounterValueKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_wait_semaphores: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkWaitSemaphores\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkWaitSemaphores".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_timeline_semaphore {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkWaitSemaphoresKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_signal_semaphore: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSignalSemaphore\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkSignalSemaphore".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_timeline_semaphore {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSignalSemaphoreKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_android_hardware_buffer_properties_android: if extensions
                .android_external_memory_android_hardware_buffer
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAndroidHardwareBufferPropertiesANDROID\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_memory_android_hardware_buffer_android: if extensions.android_external_memory_android_hardware_buffer
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryAndroidHardwareBufferANDROID\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_draw_indirect_count: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCount\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDrawIndirectCount".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_draw_indirect_count {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else if extensions.amd_draw_indirect_count {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountAMD\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_draw_indexed_indirect_count: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexedIndirectCount\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkCmdDrawIndexedIndirectCount".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_draw_indirect_count {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndexedIndirectCountKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else if extensions.amd_draw_indirect_count {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndexedIndirectCountAMD\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_checkpoint_nv: if extensions.nv_device_diagnostic_checkpoints {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCheckpointNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_queue_checkpoint_data_nv: if extensions.nv_device_diagnostic_checkpoints {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetQueueCheckpointDataNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_bind_transform_feedback_buffers_ext: if extensions.ext_transform_feedback {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindTransformFeedbackBuffersEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_begin_transform_feedback_ext: if extensions.ext_transform_feedback {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginTransformFeedbackEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_end_transform_feedback_ext: if extensions.ext_transform_feedback {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndTransformFeedbackEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_begin_query_indexed_ext: if extensions.ext_transform_feedback {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQueryIndexedEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_end_query_indexed_ext: if extensions.ext_transform_feedback {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQueryIndexedEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_draw_indirect_byte_count_ext: if extensions.ext_transform_feedback {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectByteCountEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_exclusive_scissor_nv: if extensions.nv_scissor_exclusive {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetExclusiveScissorNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_bind_shading_rate_image_nv: if extensions.nv_shading_rate_image {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindShadingRateImageNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_viewport_shading_rate_palette_nv: if extensions.nv_shading_rate_image {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportShadingRatePaletteNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_coarse_sample_order_nv: if extensions.nv_shading_rate_image {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCoarseSampleOrderNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_draw_mesh_tasks_nv: if extensions.nv_mesh_shader {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_draw_mesh_tasks_indirect_nv: if extensions.nv_mesh_shader {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksIndirectNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_draw_mesh_tasks_indirect_count_nv: if extensions.nv_mesh_shader {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawMeshTasksIndirectCountNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_compile_deferred_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCompileDeferredNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_acceleration_structure_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateAccelerationStructureNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_acceleration_structure_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyAccelerationStructureKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_acceleration_structure_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyAccelerationStructureNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_acceleration_structure_memory_requirements_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureMemoryRequirementsNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_bind_acceleration_structure_memory_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkBindAccelerationStructureMemoryNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_copy_acceleration_structure_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_copy_acceleration_structure_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_copy_acceleration_structure_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCopyAccelerationStructureKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_copy_acceleration_structure_to_memory_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureToMemoryKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_copy_acceleration_structure_to_memory_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCopyAccelerationStructureToMemoryKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_copy_memory_to_acceleration_structure_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyMemoryToAccelerationStructureKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_copy_memory_to_acceleration_structure_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCopyMemoryToAccelerationStructureKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_write_acceleration_structures_properties_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteAccelerationStructuresPropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_write_acceleration_structures_properties_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteAccelerationStructuresPropertiesNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_build_acceleration_structure_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructureNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_write_acceleration_structures_properties_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkWriteAccelerationStructuresPropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_trace_rays_khr: if extensions.khr_ray_tracing_pipeline {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_trace_rays_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_ray_tracing_shader_group_handles_khr: if extensions.khr_ray_tracing_pipeline {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupHandlesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupHandlesNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_ray_tracing_capture_replay_shader_group_handles_khr: if extensions.khr_ray_tracing_pipeline {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_acceleration_structure_handle_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureHandleNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_ray_tracing_pipelines_nv: if extensions.nv_ray_tracing {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateRayTracingPipelinesNV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_ray_tracing_pipelines_khr: if extensions.khr_ray_tracing_pipeline {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateRayTracingPipelinesKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_cooperative_matrix_properties_nv: if extensions.nv_cooperative_matrix {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_trace_rays_indirect_khr: if extensions.khr_ray_tracing_pipeline {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysIndirectKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_device_acceleration_structure_compatibility_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceAccelerationStructureCompatibilityKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_ray_tracing_shader_group_stack_size_khr: if extensions.khr_ray_tracing_pipeline {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupStackSizeKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_ray_tracing_pipeline_stack_size_khr: if extensions.khr_ray_tracing_pipeline {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRayTracingPipelineStackSizeKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_image_view_handle_nvx: if extensions.nvx_image_view_handle {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewHandleNVX\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_image_view_address_nvx: if extensions.nvx_image_view_handle {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewAddressNVX\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_surface_present_modes2_ext: if extensions.ext_full_screen_exclusive {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfacePresentModes2EXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_device_group_surface_present_modes2_ext: if (extensions.ext_full_screen_exclusive
                && extensions.khr_device_group)
                || (extensions.ext_full_screen_exclusive && version >= vk::Version::from_raw_parts(1, 1, 0))
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupSurfacePresentModes2EXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_acquire_full_screen_exclusive_mode_ext: if extensions.ext_full_screen_exclusive {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkAcquireFullScreenExclusiveModeEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_release_full_screen_exclusive_mode_ext: if extensions.ext_full_screen_exclusive {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkReleaseFullScreenExclusiveModeEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_enumerate_physical_device_queue_family_performance_query_counters_khr: if extensions
                .khr_performance_query
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_queue_family_performance_query_passes_khr: if extensions.khr_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_acquire_profiling_lock_khr: if extensions.khr_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkAcquireProfilingLockKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_release_profiling_lock_khr: if extensions.khr_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkReleaseProfilingLockKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_image_drm_format_modifier_properties_ext: if extensions.ext_image_drm_format_modifier {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageDrmFormatModifierPropertiesEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_buffer_opaque_capture_address: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferOpaqueCaptureAddress\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetBufferOpaqueCaptureAddress".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_buffer_device_address {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferOpaqueCaptureAddressKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_buffer_device_address: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddress\0"));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol("vkGetBufferDeviceAddress".to_string()));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_buffer_device_address {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddressKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else if extensions.ext_buffer_device_address {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddressEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: if extensions
                .nv_coverage_reduction_mode
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_initialize_performance_api_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkInitializePerformanceApiINTEL\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_uninitialize_performance_api_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkUninitializePerformanceApiINTEL\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_performance_marker_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPerformanceMarkerINTEL\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_performance_stream_marker_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPerformanceStreamMarkerINTEL\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_performance_override_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPerformanceOverrideINTEL\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_acquire_performance_configuration_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkAcquirePerformanceConfigurationINTEL\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_release_performance_configuration_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkReleasePerformanceConfigurationINTEL\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_queue_set_performance_configuration_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueSetPerformanceConfigurationINTEL\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_performance_parameter_intel: if extensions.intel_performance_query {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetPerformanceParameterINTEL\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_device_memory_opaque_capture_address: if version >= vk::Version::from_raw_parts(1, 2, 0) {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceMemoryOpaqueCaptureAddress\0",
                ));
                if fp.is_none() {
                    return Err(LoaderError::MissingSymbol(
                        "vkGetDeviceMemoryOpaqueCaptureAddress".to_string(),
                    ));
                }
                fp.map(|f| mem::transmute(f))
            } else if extensions.khr_buffer_device_address {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceMemoryOpaqueCaptureAddressKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_pipeline_executable_properties_khr: if extensions.khr_pipeline_executable_properties {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutablePropertiesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_pipeline_executable_statistics_khr: if extensions.khr_pipeline_executable_properties {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutableStatisticsKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_pipeline_executable_internal_representations_khr: if extensions.khr_pipeline_executable_properties {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutableInternalRepresentationsKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_line_stipple_ext: if extensions.ext_line_rasterization {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_tool_properties_ext: if extensions.ext_tooling_info {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceToolPropertiesEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_acceleration_structure_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateAccelerationStructureKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_build_acceleration_structures_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructuresKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_build_acceleration_structures_indirect_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructuresIndirectKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_build_acceleration_structures_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkBuildAccelerationStructuresKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_acceleration_structure_device_address_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureDeviceAddressKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_deferred_operation_khr: if extensions.khr_deferred_host_operations {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDeferredOperationKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_deferred_operation_khr: if extensions.khr_deferred_host_operations {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyDeferredOperationKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_deferred_operation_max_concurrency_khr: if extensions.khr_deferred_host_operations {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeferredOperationMaxConcurrencyKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_deferred_operation_result_khr: if extensions.khr_deferred_host_operations {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeferredOperationResultKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_deferred_operation_join_khr: if extensions.khr_deferred_host_operations {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDeferredOperationJoinKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_cull_mode_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullModeEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_front_face_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFaceEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_primitive_topology_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPrimitiveTopologyEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_viewport_with_count_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWithCountEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_scissor_with_count_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissorWithCountEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_bind_vertex_buffers2_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers2EXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_depth_test_enable_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthTestEnableEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_depth_write_enable_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthWriteEnableEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_depth_compare_op_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOpEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_depth_bounds_test_enable_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthBoundsTestEnableEXT\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_stencil_test_enable_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilTestEnableEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_stencil_op_ext: if extensions.ext_extended_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOpEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_create_private_data_slot_ext: if extensions.ext_private_data {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCreatePrivateDataSlotEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_destroy_private_data_slot_ext: if extensions.ext_private_data {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyPrivateDataSlotEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_set_private_data_ext: if extensions.ext_private_data {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkSetPrivateDataEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_private_data_ext: if extensions.ext_private_data {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetPrivateDataEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_copy_buffer2_khr: if extensions.khr_copy_commands2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_copy_image2_khr: if extensions.khr_copy_commands2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_blit_image2_khr: if extensions.khr_copy_commands2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_copy_buffer_to_image2_khr: if extensions.khr_copy_commands2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBufferToImage2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_copy_image_to_buffer2_khr: if extensions.khr_copy_commands2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImageToBuffer2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_resolve_image2_khr: if extensions.khr_copy_commands2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_fragment_shading_rate_khr: if extensions.khr_fragment_shading_rate {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFragmentShadingRateKHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_physical_device_fragment_shading_rates_khr: if extensions.khr_fragment_shading_rate {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFragmentShadingRatesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_fragment_shading_rate_enum_nv: if extensions.nv_fragment_shading_rate_enums {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetFragmentShadingRateEnumNV\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_acceleration_structure_build_sizes_khr: if extensions.khr_acceleration_structure {
                let fp = f(CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureBuildSizesKHR\0",
                ));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_vertex_input_ext: if extensions.ext_vertex_input_dynamic_state {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetVertexInputEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_color_write_enable_ext: if extensions.ext_color_write_enable {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorWriteEnableEXT\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_set_event2_khr: if extensions.khr_synchronization2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_reset_event2_khr: if extensions.khr_synchronization2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_wait_events2_khr: if extensions.khr_synchronization2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_pipeline_barrier2_khr: if extensions.khr_synchronization2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_queue_submit2_khr: if extensions.khr_synchronization2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_write_timestamp2_khr: if extensions.khr_synchronization2 {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp2KHR\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_cmd_write_buffer_marker2_amd: if extensions.khr_synchronization2 && extensions.amd_buffer_marker {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteBufferMarker2AMD\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
            fp_get_queue_checkpoint_data2_nv: if extensions.khr_synchronization2
                && extensions.nv_device_diagnostic_checkpoints
            {
                let fp = f(CStr::from_bytes_with_nul_unchecked(b"vkGetQueueCheckpointData2NV\0"));
                fp.map(|f| mem::transmute(f))
            } else {
                None
            },
        })
    }
    pub unsafe fn destroy_device(&self, p_allocator: Option<&vk::AllocationCallbacks>) {
        let fp = self.fp_destroy_device.expect("vkDestroyDevice is not loaded");
        (fp)(Some(self.handle), p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> vk::Queue {
        let fp = self.fp_get_device_queue.expect("vkGetDeviceQueue is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), queue_family_index, queue_index, res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn queue_submit(
        &self,
        queue: vk::Queue,
        p_submits: &[vk::SubmitInfo],
        fence: Option<vk::Fence>,
    ) -> Result<()> {
        let fp = self.fp_queue_submit.expect("vkQueueSubmit is not loaded");
        let submit_count = p_submits.len() as u32;
        let err = (fp)(
            Some(queue),
            submit_count,
            p_submits.first().map_or(ptr::null(), |s| s as *const _),
            fence,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn queue_wait_idle(&self, queue: vk::Queue) -> Result<()> {
        let fp = self.fp_queue_wait_idle.expect("vkQueueWaitIdle is not loaded");
        let err = (fp)(Some(queue));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn device_wait_idle(&self) -> Result<()> {
        let fp = self.fp_device_wait_idle.expect("vkDeviceWaitIdle is not loaded");
        let err = (fp)(Some(self.handle));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn allocate_memory(
        &self,
        p_allocate_info: &vk::MemoryAllocateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DeviceMemory> {
        let fp = self.fp_allocate_memory.expect("vkAllocateMemory is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_allocate_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn free_memory(&self, memory: Option<vk::DeviceMemory>, p_allocator: Option<&vk::AllocationCallbacks>) {
        let fp = self.fp_free_memory.expect("vkFreeMemory is not loaded");
        (fp)(Some(self.handle), memory, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn map_memory(
        &self,
        memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        flags: vk::MemoryMapFlags,
    ) -> Result<*mut c_void> {
        let fp = self.fp_map_memory.expect("vkMapMemory is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), Some(memory), offset, size, flags, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn unmap_memory(&self, memory: vk::DeviceMemory) {
        let fp = self.fp_unmap_memory.expect("vkUnmapMemory is not loaded");
        (fp)(Some(self.handle), Some(memory));
    }
    pub unsafe fn flush_mapped_memory_ranges(&self, p_memory_ranges: &[vk::MappedMemoryRange]) -> Result<()> {
        let fp = self
            .fp_flush_mapped_memory_ranges
            .expect("vkFlushMappedMemoryRanges is not loaded");
        let memory_range_count = p_memory_ranges.len() as u32;
        let err = (fp)(
            Some(self.handle),
            memory_range_count,
            p_memory_ranges.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn invalidate_mapped_memory_ranges(&self, p_memory_ranges: &[vk::MappedMemoryRange]) -> Result<()> {
        let fp = self
            .fp_invalidate_mapped_memory_ranges
            .expect("vkInvalidateMappedMemoryRanges is not loaded");
        let memory_range_count = p_memory_ranges.len() as u32;
        let err = (fp)(
            Some(self.handle),
            memory_range_count,
            p_memory_ranges.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_device_memory_commitment(&self, memory: vk::DeviceMemory) -> vk::DeviceSize {
        let fp = self
            .fp_get_device_memory_commitment
            .expect("vkGetDeviceMemoryCommitment is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), Some(memory), res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn get_buffer_memory_requirements(&self, buffer: vk::Buffer) -> vk::MemoryRequirements {
        let fp = self
            .fp_get_buffer_memory_requirements
            .expect("vkGetBufferMemoryRequirements is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), Some(buffer), res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        buffer: vk::Buffer,
        memory: vk::DeviceMemory,
        memory_offset: vk::DeviceSize,
    ) -> Result<()> {
        let fp = self.fp_bind_buffer_memory.expect("vkBindBufferMemory is not loaded");
        let err = (fp)(Some(self.handle), Some(buffer), Some(memory), memory_offset);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_image_memory_requirements(&self, image: vk::Image) -> vk::MemoryRequirements {
        let fp = self
            .fp_get_image_memory_requirements
            .expect("vkGetImageMemoryRequirements is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), Some(image), res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn bind_image_memory(
        &self,
        image: vk::Image,
        memory: vk::DeviceMemory,
        memory_offset: vk::DeviceSize,
    ) -> Result<()> {
        let fp = self.fp_bind_image_memory.expect("vkBindImageMemory is not loaded");
        let err = (fp)(Some(self.handle), Some(image), Some(memory), memory_offset);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_image_sparse_memory_requirements_to_vec(
        &self,
        image: vk::Image,
    ) -> Vec<vk::SparseImageMemoryRequirements> {
        let fp = self
            .fp_get_image_sparse_memory_requirements
            .expect("vkGetImageSparseMemoryRequirements is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), Some(image), len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(self.handle), Some(image), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: vk::Queue,
        p_bind_info: &[vk::BindSparseInfo],
        fence: Option<vk::Fence>,
    ) -> Result<()> {
        let fp = self.fp_queue_bind_sparse.expect("vkQueueBindSparse is not loaded");
        let bind_info_count = p_bind_info.len() as u32;
        let err = (fp)(
            Some(queue),
            bind_info_count,
            p_bind_info.first().map_or(ptr::null(), |s| s as *const _),
            fence,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_fence(
        &self,
        p_create_info: &vk::FenceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Fence> {
        let fp = self.fp_create_fence.expect("vkCreateFence is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_fence(&self, fence: Option<vk::Fence>, p_allocator: Option<&vk::AllocationCallbacks>) {
        let fp = self.fp_destroy_fence.expect("vkDestroyFence is not loaded");
        (fp)(Some(self.handle), fence, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn reset_fences(&self, p_fences: &[vk::Fence]) -> Result<()> {
        let fp = self.fp_reset_fences.expect("vkResetFences is not loaded");
        let fence_count = p_fences.len() as u32;
        let err = (fp)(
            Some(self.handle),
            fence_count,
            p_fences.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_fence_status(&self, fence: vk::Fence) -> Result<vk::Result> {
        let fp = self.fp_get_fence_status.expect("vkGetFenceStatus is not loaded");
        let err = (fp)(Some(self.handle), Some(fence));
        match err {
            vk::Result::SUCCESS | vk::Result::NOT_READY => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn wait_for_fences(&self, p_fences: &[vk::Fence], wait_all: bool, timeout: u64) -> Result<vk::Result> {
        let fp = self.fp_wait_for_fences.expect("vkWaitForFences is not loaded");
        let fence_count = p_fences.len() as u32;
        let err = (fp)(
            Some(self.handle),
            fence_count,
            p_fences.first().map_or(ptr::null(), |s| s as *const _),
            if wait_all { vk::TRUE } else { vk::FALSE },
            timeout,
        );
        match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn create_semaphore(
        &self,
        p_create_info: &vk::SemaphoreCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Semaphore> {
        let fp = self.fp_create_semaphore.expect("vkCreateSemaphore is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_semaphore(
        &self,
        semaphore: Option<vk::Semaphore>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self.fp_destroy_semaphore.expect("vkDestroySemaphore is not loaded");
        (fp)(Some(self.handle), semaphore, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_event(
        &self,
        p_create_info: &vk::EventCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Event> {
        let fp = self.fp_create_event.expect("vkCreateEvent is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_event(&self, event: Option<vk::Event>, p_allocator: Option<&vk::AllocationCallbacks>) {
        let fp = self.fp_destroy_event.expect("vkDestroyEvent is not loaded");
        (fp)(Some(self.handle), event, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_event_status(&self, event: vk::Event) -> Result<vk::Result> {
        let fp = self.fp_get_event_status.expect("vkGetEventStatus is not loaded");
        let err = (fp)(Some(self.handle), Some(event));
        match err {
            vk::Result::EVENT_SET | vk::Result::EVENT_RESET => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn set_event(&self, event: vk::Event) -> Result<()> {
        let fp = self.fp_set_event.expect("vkSetEvent is not loaded");
        let err = (fp)(Some(self.handle), Some(event));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn reset_event(&self, event: vk::Event) -> Result<()> {
        let fp = self.fp_reset_event.expect("vkResetEvent is not loaded");
        let err = (fp)(Some(self.handle), Some(event));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_query_pool(
        &self,
        p_create_info: &vk::QueryPoolCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::QueryPool> {
        let fp = self.fp_create_query_pool.expect("vkCreateQueryPool is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_query_pool(
        &self,
        query_pool: Option<vk::QueryPool>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self.fp_destroy_query_pool.expect("vkDestroyQueryPool is not loaded");
        (fp)(Some(self.handle), query_pool, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_query_pool_results<T>(
        &self,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
        p_data: &mut [T],
        stride: vk::DeviceSize,
        flags: vk::QueryResultFlags,
    ) -> Result<vk::Result> {
        let fp = self
            .fp_get_query_pool_results
            .expect("vkGetQueryPoolResults is not loaded");
        let data_size = mem::size_of_val(p_data) as usize;
        let err = (fp)(
            Some(self.handle),
            Some(query_pool),
            first_query,
            query_count,
            data_size,
            p_data.first_mut().map_or(ptr::null_mut(), |s| s as *mut _) as *mut _,
            stride,
            flags,
        );
        match err {
            vk::Result::SUCCESS | vk::Result::NOT_READY => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn reset_query_pool(&self, query_pool: vk::QueryPool, first_query: u32, query_count: u32) {
        let fp = self.fp_reset_query_pool.expect("vkResetQueryPool is not loaded");
        (fp)(Some(self.handle), Some(query_pool), first_query, query_count);
    }
    pub unsafe fn reset_query_pool_ext(&self, query_pool: vk::QueryPool, first_query: u32, query_count: u32) {
        let fp = self.fp_reset_query_pool.expect("vkResetQueryPoolEXT is not loaded");
        (fp)(Some(self.handle), Some(query_pool), first_query, query_count);
    }
    pub unsafe fn create_buffer(
        &self,
        p_create_info: &vk::BufferCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Buffer> {
        let fp = self.fp_create_buffer.expect("vkCreateBuffer is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_buffer(&self, buffer: Option<vk::Buffer>, p_allocator: Option<&vk::AllocationCallbacks>) {
        let fp = self.fp_destroy_buffer.expect("vkDestroyBuffer is not loaded");
        (fp)(Some(self.handle), buffer, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_buffer_view(
        &self,
        p_create_info: &vk::BufferViewCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::BufferView> {
        let fp = self.fp_create_buffer_view.expect("vkCreateBufferView is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_buffer_view(
        &self,
        buffer_view: Option<vk::BufferView>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self.fp_destroy_buffer_view.expect("vkDestroyBufferView is not loaded");
        (fp)(Some(self.handle), buffer_view, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_image(
        &self,
        p_create_info: &vk::ImageCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Image> {
        let fp = self.fp_create_image.expect("vkCreateImage is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_image(&self, image: Option<vk::Image>, p_allocator: Option<&vk::AllocationCallbacks>) {
        let fp = self.fp_destroy_image.expect("vkDestroyImage is not loaded");
        (fp)(Some(self.handle), image, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_image_subresource_layout(
        &self,
        image: vk::Image,
        p_subresource: &vk::ImageSubresource,
    ) -> vk::SubresourceLayout {
        let fp = self
            .fp_get_image_subresource_layout
            .expect("vkGetImageSubresourceLayout is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), Some(image), p_subresource, res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn create_image_view(
        &self,
        p_create_info: &vk::ImageViewCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ImageView> {
        let fp = self.fp_create_image_view.expect("vkCreateImageView is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_image_view(
        &self,
        image_view: Option<vk::ImageView>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self.fp_destroy_image_view.expect("vkDestroyImageView is not loaded");
        (fp)(Some(self.handle), image_view, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_shader_module(
        &self,
        p_create_info: &vk::ShaderModuleCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ShaderModule> {
        let fp = self
            .fp_create_shader_module
            .expect("vkCreateShaderModule is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_shader_module(
        &self,
        shader_module: Option<vk::ShaderModule>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_shader_module
            .expect("vkDestroyShaderModule is not loaded");
        (fp)(Some(self.handle), shader_module, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_pipeline_cache(
        &self,
        p_create_info: &vk::PipelineCacheCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::PipelineCache> {
        let fp = self
            .fp_create_pipeline_cache
            .expect("vkCreatePipelineCache is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_pipeline_cache(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_pipeline_cache
            .expect("vkDestroyPipelineCache is not loaded");
        (fp)(
            Some(self.handle),
            pipeline_cache,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_pipeline_cache_data(
        &self,
        pipeline_cache: vk::PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result<vk::Result> {
        let fp = self
            .fp_get_pipeline_cache_data
            .expect("vkGetPipelineCacheData is not loaded");
        let err = (fp)(Some(self.handle), Some(pipeline_cache), p_data_size, p_data);
        match err {
            vk::Result::SUCCESS | vk::Result::INCOMPLETE => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn merge_pipeline_caches(
        &self,
        dst_cache: vk::PipelineCache,
        p_src_caches: &[vk::PipelineCache],
    ) -> Result<()> {
        let fp = self
            .fp_merge_pipeline_caches
            .expect("vkMergePipelineCaches is not loaded");
        let src_cache_count = p_src_caches.len() as u32;
        let err = (fp)(
            Some(self.handle),
            Some(dst_cache),
            src_cache_count,
            p_src_caches.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_graphics_pipelines(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::GraphicsPipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<()> {
        let fp = self
            .fp_create_graphics_pipelines
            .expect("vkCreateGraphicsPipelines is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            p_pipelines,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_graphics_pipelines_to_vec(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::GraphicsPipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        let fp = self
            .fp_create_graphics_pipelines
            .expect("vkCreateGraphicsPipelines is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(create_info_count as usize);
        v.set_len(create_info_count as usize);
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_graphics_pipelines_array<const N: usize>(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::GraphicsPipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<[vk::Pipeline; N]> {
        let fp = self
            .fp_create_graphics_pipelines
            .expect("vkCreateGraphicsPipelines is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, N as u32);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr() as *mut _,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_graphics_pipelines_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &vk::GraphicsPipelineCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        let fp = self
            .fp_create_graphics_pipelines
            .expect("vkCreateGraphicsPipelines is not loaded");
        let create_info_count = 1;
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_compute_pipelines(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<()> {
        let fp = self
            .fp_create_compute_pipelines
            .expect("vkCreateComputePipelines is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            p_pipelines,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_compute_pipelines_to_vec(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        let fp = self
            .fp_create_compute_pipelines
            .expect("vkCreateComputePipelines is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(create_info_count as usize);
        v.set_len(create_info_count as usize);
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_compute_pipelines_array<const N: usize>(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<[vk::Pipeline; N]> {
        let fp = self
            .fp_create_compute_pipelines
            .expect("vkCreateComputePipelines is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, N as u32);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr() as *mut _,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_compute_pipelines_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &vk::ComputePipelineCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        let fp = self
            .fp_create_compute_pipelines
            .expect("vkCreateComputePipelines is not loaded");
        let create_info_count = 1;
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn destroy_pipeline(
        &self,
        pipeline: Option<vk::Pipeline>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self.fp_destroy_pipeline.expect("vkDestroyPipeline is not loaded");
        (fp)(Some(self.handle), pipeline, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_pipeline_layout(
        &self,
        p_create_info: &vk::PipelineLayoutCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::PipelineLayout> {
        let fp = self
            .fp_create_pipeline_layout
            .expect("vkCreatePipelineLayout is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_pipeline_layout(
        &self,
        pipeline_layout: Option<vk::PipelineLayout>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_pipeline_layout
            .expect("vkDestroyPipelineLayout is not loaded");
        (fp)(
            Some(self.handle),
            pipeline_layout,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn create_sampler(
        &self,
        p_create_info: &vk::SamplerCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Sampler> {
        let fp = self.fp_create_sampler.expect("vkCreateSampler is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_sampler(&self, sampler: Option<vk::Sampler>, p_allocator: Option<&vk::AllocationCallbacks>) {
        let fp = self.fp_destroy_sampler.expect("vkDestroySampler is not loaded");
        (fp)(Some(self.handle), sampler, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_descriptor_set_layout(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorSetLayout> {
        let fp = self
            .fp_create_descriptor_set_layout
            .expect("vkCreateDescriptorSetLayout is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: Option<vk::DescriptorSetLayout>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_descriptor_set_layout
            .expect("vkDestroyDescriptorSetLayout is not loaded");
        (fp)(
            Some(self.handle),
            descriptor_set_layout,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn create_descriptor_pool(
        &self,
        p_create_info: &vk::DescriptorPoolCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorPool> {
        let fp = self
            .fp_create_descriptor_pool
            .expect("vkCreateDescriptorPool is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_descriptor_pool(
        &self,
        descriptor_pool: Option<vk::DescriptorPool>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_descriptor_pool
            .expect("vkDestroyDescriptorPool is not loaded");
        (fp)(
            Some(self.handle),
            descriptor_pool,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn reset_descriptor_pool(
        &self,
        descriptor_pool: vk::DescriptorPool,
        flags: vk::DescriptorPoolResetFlags,
    ) -> Result<()> {
        let fp = self
            .fp_reset_descriptor_pool
            .expect("vkResetDescriptorPool is not loaded");
        let err = (fp)(Some(self.handle), Some(descriptor_pool), flags);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn allocate_descriptor_sets(
        &self,
        p_allocate_info: &vk::DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut vk::DescriptorSet,
    ) -> Result<()> {
        let fp = self
            .fp_allocate_descriptor_sets
            .expect("vkAllocateDescriptorSets is not loaded");
        let v_err = (fp)(Some(self.handle), p_allocate_info, p_descriptor_sets);
        match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn allocate_descriptor_sets_to_vec(
        &self,
        p_allocate_info: &vk::DescriptorSetAllocateInfo,
    ) -> Result<Vec<vk::DescriptorSet>> {
        let fp = self
            .fp_allocate_descriptor_sets
            .expect("vkAllocateDescriptorSets is not loaded");
        let mut v = Vec::with_capacity(p_allocate_info.descriptor_set_count as usize);
        v.set_len(p_allocate_info.descriptor_set_count as usize);
        let v_err = (fp)(Some(self.handle), p_allocate_info, v.as_mut_ptr());
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn allocate_descriptor_sets_array<const N: usize>(
        &self,
        p_allocate_info: &vk::DescriptorSetAllocateInfo,
    ) -> Result<[vk::DescriptorSet; N]> {
        let fp = self
            .fp_allocate_descriptor_sets
            .expect("vkAllocateDescriptorSets is not loaded");
        assert_eq!(p_allocate_info.descriptor_set_count, N as u32);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(Some(self.handle), p_allocate_info, v.as_mut_ptr() as *mut _);
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn allocate_descriptor_sets_single(
        &self,
        p_allocate_info: &vk::DescriptorSetAllocateInfo,
    ) -> Result<vk::DescriptorSet> {
        let fp = self
            .fp_allocate_descriptor_sets
            .expect("vkAllocateDescriptorSets is not loaded");
        assert_eq!(p_allocate_info.descriptor_set_count, 1);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(Some(self.handle), p_allocate_info, v.as_mut_ptr());
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn free_descriptor_sets(
        &self,
        descriptor_pool: vk::DescriptorPool,
        p_descriptor_sets: &[vk::DescriptorSet],
    ) -> Result<()> {
        let fp = self
            .fp_free_descriptor_sets
            .expect("vkFreeDescriptorSets is not loaded");
        let descriptor_set_count = p_descriptor_sets.len() as u32;
        let err = (fp)(
            Some(self.handle),
            Some(descriptor_pool),
            descriptor_set_count,
            p_descriptor_sets.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn update_descriptor_sets(
        &self,
        p_descriptor_writes: &[vk::WriteDescriptorSet],
        p_descriptor_copies: &[vk::CopyDescriptorSet],
    ) {
        let fp = self
            .fp_update_descriptor_sets
            .expect("vkUpdateDescriptorSets is not loaded");
        let descriptor_write_count = p_descriptor_writes.len() as u32;
        let descriptor_copy_count = p_descriptor_copies.len() as u32;
        (fp)(
            Some(self.handle),
            descriptor_write_count,
            p_descriptor_writes.first().map_or(ptr::null(), |s| s as *const _),
            descriptor_copy_count,
            p_descriptor_copies.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn create_framebuffer(
        &self,
        p_create_info: &vk::FramebufferCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Framebuffer> {
        let fp = self.fp_create_framebuffer.expect("vkCreateFramebuffer is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_framebuffer(
        &self,
        framebuffer: Option<vk::Framebuffer>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self.fp_destroy_framebuffer.expect("vkDestroyFramebuffer is not loaded");
        (fp)(Some(self.handle), framebuffer, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_render_pass(
        &self,
        p_create_info: &vk::RenderPassCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::RenderPass> {
        let fp = self.fp_create_render_pass.expect("vkCreateRenderPass is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_render_pass(
        &self,
        render_pass: Option<vk::RenderPass>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self.fp_destroy_render_pass.expect("vkDestroyRenderPass is not loaded");
        (fp)(Some(self.handle), render_pass, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_render_area_granularity(&self, render_pass: vk::RenderPass) -> vk::Extent2D {
        let fp = self
            .fp_get_render_area_granularity
            .expect("vkGetRenderAreaGranularity is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), Some(render_pass), res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn create_command_pool(
        &self,
        p_create_info: &vk::CommandPoolCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::CommandPool> {
        let fp = self.fp_create_command_pool.expect("vkCreateCommandPool is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_command_pool(
        &self,
        command_pool: Option<vk::CommandPool>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_command_pool
            .expect("vkDestroyCommandPool is not loaded");
        (fp)(Some(self.handle), command_pool, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn reset_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolResetFlags,
    ) -> Result<()> {
        let fp = self.fp_reset_command_pool.expect("vkResetCommandPool is not loaded");
        let err = (fp)(Some(self.handle), Some(command_pool), flags);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn allocate_command_buffers(
        &self,
        p_allocate_info: &vk::CommandBufferAllocateInfo,
        p_command_buffers: *mut vk::CommandBuffer,
    ) -> Result<()> {
        let fp = self
            .fp_allocate_command_buffers
            .expect("vkAllocateCommandBuffers is not loaded");
        let v_err = (fp)(Some(self.handle), p_allocate_info, p_command_buffers);
        match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn allocate_command_buffers_to_vec(
        &self,
        p_allocate_info: &vk::CommandBufferAllocateInfo,
    ) -> Result<Vec<vk::CommandBuffer>> {
        let fp = self
            .fp_allocate_command_buffers
            .expect("vkAllocateCommandBuffers is not loaded");
        let mut v = Vec::with_capacity(p_allocate_info.command_buffer_count as usize);
        v.set_len(p_allocate_info.command_buffer_count as usize);
        let v_err = (fp)(Some(self.handle), p_allocate_info, v.as_mut_ptr());
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn allocate_command_buffers_array<const N: usize>(
        &self,
        p_allocate_info: &vk::CommandBufferAllocateInfo,
    ) -> Result<[vk::CommandBuffer; N]> {
        let fp = self
            .fp_allocate_command_buffers
            .expect("vkAllocateCommandBuffers is not loaded");
        assert_eq!(p_allocate_info.command_buffer_count, N as u32);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(Some(self.handle), p_allocate_info, v.as_mut_ptr() as *mut _);
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn allocate_command_buffers_single(
        &self,
        p_allocate_info: &vk::CommandBufferAllocateInfo,
    ) -> Result<vk::CommandBuffer> {
        let fp = self
            .fp_allocate_command_buffers
            .expect("vkAllocateCommandBuffers is not loaded");
        assert_eq!(p_allocate_info.command_buffer_count, 1);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(Some(self.handle), p_allocate_info, v.as_mut_ptr());
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn free_command_buffers(&self, command_pool: vk::CommandPool, p_command_buffers: &[vk::CommandBuffer]) {
        let fp = self
            .fp_free_command_buffers
            .expect("vkFreeCommandBuffers is not loaded");
        let command_buffer_count = p_command_buffers.len() as u32;
        (fp)(
            Some(self.handle),
            Some(command_pool),
            command_buffer_count,
            p_command_buffers.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        p_begin_info: &vk::CommandBufferBeginInfo,
    ) -> Result<()> {
        let fp = self
            .fp_begin_command_buffer
            .expect("vkBeginCommandBuffer is not loaded");
        let err = (fp)(Some(command_buffer), p_begin_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
        let fp = self.fp_end_command_buffer.expect("vkEndCommandBuffer is not loaded");
        let err = (fp)(Some(command_buffer));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        flags: vk::CommandBufferResetFlags,
    ) -> Result<()> {
        let fp = self
            .fp_reset_command_buffer
            .expect("vkResetCommandBuffer is not loaded");
        let err = (fp)(Some(command_buffer), flags);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        let fp = self.fp_cmd_bind_pipeline.expect("vkCmdBindPipeline is not loaded");
        (fp)(Some(command_buffer), pipeline_bind_point, Some(pipeline));
    }
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_viewports: &[vk::Viewport],
    ) {
        let fp = self.fp_cmd_set_viewport.expect("vkCmdSetViewport is not loaded");
        let viewport_count = p_viewports.len() as u32;
        (fp)(
            Some(command_buffer),
            first_viewport,
            viewport_count,
            p_viewports.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: vk::CommandBuffer,
        first_scissor: u32,
        p_scissors: &[vk::Rect2D],
    ) {
        let fp = self.fp_cmd_set_scissor.expect("vkCmdSetScissor is not loaded");
        let scissor_count = p_scissors.len() as u32;
        (fp)(
            Some(command_buffer),
            first_scissor,
            scissor_count,
            p_scissors.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_line_width(&self, command_buffer: vk::CommandBuffer, line_width: f32) {
        let fp = self.fp_cmd_set_line_width.expect("vkCmdSetLineWidth is not loaded");
        (fp)(Some(command_buffer), line_width);
    }
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        let fp = self.fp_cmd_set_depth_bias.expect("vkCmdSetDepthBias is not loaded");
        (fp)(
            Some(command_buffer),
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        );
    }
    pub unsafe fn cmd_set_blend_constants(&self, command_buffer: vk::CommandBuffer, blend_constants: [f32; 4]) {
        let fp = self
            .fp_cmd_set_blend_constants
            .expect("vkCmdSetBlendConstants is not loaded");
        (fp)(Some(command_buffer), blend_constants.as_ptr());
    }
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: vk::CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        let fp = self.fp_cmd_set_depth_bounds.expect("vkCmdSetDepthBounds is not loaded");
        (fp)(Some(command_buffer), min_depth_bounds, max_depth_bounds);
    }
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        compare_mask: u32,
    ) {
        let fp = self
            .fp_cmd_set_stencil_compare_mask
            .expect("vkCmdSetStencilCompareMask is not loaded");
        (fp)(Some(command_buffer), face_mask, compare_mask);
    }
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        write_mask: u32,
    ) {
        let fp = self
            .fp_cmd_set_stencil_write_mask
            .expect("vkCmdSetStencilWriteMask is not loaded");
        (fp)(Some(command_buffer), face_mask, write_mask);
    }
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        reference: u32,
    ) {
        let fp = self
            .fp_cmd_set_stencil_reference
            .expect("vkCmdSetStencilReference is not loaded");
        (fp)(Some(command_buffer), face_mask, reference);
    }
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        p_descriptor_sets: &[vk::DescriptorSet],
        p_dynamic_offsets: &[u32],
    ) {
        let fp = self
            .fp_cmd_bind_descriptor_sets
            .expect("vkCmdBindDescriptorSets is not loaded");
        let descriptor_set_count = p_descriptor_sets.len() as u32;
        let dynamic_offset_count = p_dynamic_offsets.len() as u32;
        (fp)(
            Some(command_buffer),
            pipeline_bind_point,
            Some(layout),
            first_set,
            descriptor_set_count,
            p_descriptor_sets.first().map_or(ptr::null(), |s| s as *const _),
            dynamic_offset_count,
            p_dynamic_offsets.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        index_type: vk::IndexType,
    ) {
        let fp = self
            .fp_cmd_bind_index_buffer
            .expect("vkCmdBindIndexBuffer is not loaded");
        (fp)(Some(command_buffer), Some(buffer), offset, index_type);
    }
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        p_buffers: &[vk::Buffer],
        p_offsets: &[vk::DeviceSize],
    ) {
        let fp = self
            .fp_cmd_bind_vertex_buffers
            .expect("vkCmdBindVertexBuffers is not loaded");
        let binding_count = p_buffers.len() as u32;
        assert_eq!(binding_count, p_offsets.len() as u32);
        (fp)(
            Some(command_buffer),
            first_binding,
            binding_count,
            p_buffers.first().map_or(ptr::null(), |s| s as *const _),
            p_offsets.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        let fp = self.fp_cmd_draw.expect("vkCmdDraw is not loaded");
        (fp)(
            Some(command_buffer),
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );
    }
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        let fp = self.fp_cmd_draw_indexed.expect("vkCmdDrawIndexed is not loaded");
        (fp)(
            Some(command_buffer),
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        );
    }
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let fp = self.fp_cmd_draw_indirect.expect("vkCmdDrawIndirect is not loaded");
        (fp)(Some(command_buffer), Some(buffer), offset, draw_count, stride);
    }
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_indexed_indirect
            .expect("vkCmdDrawIndexedIndirect is not loaded");
        (fp)(Some(command_buffer), Some(buffer), offset, draw_count, stride);
    }
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: vk::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        let fp = self.fp_cmd_dispatch.expect("vkCmdDispatch is not loaded");
        (fp)(Some(command_buffer), group_count_x, group_count_y, group_count_z);
    }
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
    ) {
        let fp = self
            .fp_cmd_dispatch_indirect
            .expect("vkCmdDispatchIndirect is not loaded");
        (fp)(Some(command_buffer), Some(buffer), offset);
    }
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_buffer: vk::Buffer,
        p_regions: &[vk::BufferCopy],
    ) {
        let fp = self.fp_cmd_copy_buffer.expect("vkCmdCopyBuffer is not loaded");
        let region_count = p_regions.len() as u32;
        (fp)(
            Some(command_buffer),
            Some(src_buffer),
            Some(dst_buffer),
            region_count,
            p_regions.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_regions: &[vk::ImageCopy],
    ) {
        let fp = self.fp_cmd_copy_image.expect("vkCmdCopyImage is not loaded");
        let region_count = p_regions.len() as u32;
        (fp)(
            Some(command_buffer),
            Some(src_image),
            src_image_layout,
            Some(dst_image),
            dst_image_layout,
            region_count,
            p_regions.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_regions: &[vk::ImageBlit],
        filter: vk::Filter,
    ) {
        let fp = self.fp_cmd_blit_image.expect("vkCmdBlitImage is not loaded");
        let region_count = p_regions.len() as u32;
        (fp)(
            Some(command_buffer),
            Some(src_image),
            src_image_layout,
            Some(dst_image),
            dst_image_layout,
            region_count,
            p_regions.first().map_or(ptr::null(), |s| s as *const _),
            filter,
        );
    }
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_regions: &[vk::BufferImageCopy],
    ) {
        let fp = self
            .fp_cmd_copy_buffer_to_image
            .expect("vkCmdCopyBufferToImage is not loaded");
        let region_count = p_regions.len() as u32;
        (fp)(
            Some(command_buffer),
            Some(src_buffer),
            Some(dst_image),
            dst_image_layout,
            region_count,
            p_regions.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_buffer: vk::Buffer,
        p_regions: &[vk::BufferImageCopy],
    ) {
        let fp = self
            .fp_cmd_copy_image_to_buffer
            .expect("vkCmdCopyImageToBuffer is not loaded");
        let region_count = p_regions.len() as u32;
        (fp)(
            Some(command_buffer),
            Some(src_image),
            src_image_layout,
            Some(dst_buffer),
            region_count,
            p_regions.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_update_buffer<T>(
        &self,
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        p_data: &[T],
    ) {
        let fp = self.fp_cmd_update_buffer.expect("vkCmdUpdateBuffer is not loaded");
        let data_size = mem::size_of_val(p_data) as vk::DeviceSize;
        (fp)(
            Some(command_buffer),
            Some(dst_buffer),
            dst_offset,
            data_size,
            p_data.first().map_or(ptr::null(), |s| s as *const _) as *const _,
        );
    }
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: u32,
    ) {
        let fp = self.fp_cmd_fill_buffer.expect("vkCmdFillBuffer is not loaded");
        (fp)(Some(command_buffer), Some(dst_buffer), dst_offset, size, data);
    }
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: vk::ImageLayout,
        p_color: &vk::ClearColorValue,
        p_ranges: &[vk::ImageSubresourceRange],
    ) {
        let fp = self
            .fp_cmd_clear_color_image
            .expect("vkCmdClearColorImage is not loaded");
        let range_count = p_ranges.len() as u32;
        (fp)(
            Some(command_buffer),
            Some(image),
            image_layout,
            p_color,
            range_count,
            p_ranges.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: vk::ImageLayout,
        p_depth_stencil: &vk::ClearDepthStencilValue,
        p_ranges: &[vk::ImageSubresourceRange],
    ) {
        let fp = self
            .fp_cmd_clear_depth_stencil_image
            .expect("vkCmdClearDepthStencilImage is not loaded");
        let range_count = p_ranges.len() as u32;
        (fp)(
            Some(command_buffer),
            Some(image),
            image_layout,
            p_depth_stencil,
            range_count,
            p_ranges.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: vk::CommandBuffer,
        p_attachments: &[vk::ClearAttachment],
        p_rects: &[vk::ClearRect],
    ) {
        let fp = self
            .fp_cmd_clear_attachments
            .expect("vkCmdClearAttachments is not loaded");
        let attachment_count = p_attachments.len() as u32;
        let rect_count = p_rects.len() as u32;
        (fp)(
            Some(command_buffer),
            attachment_count,
            p_attachments.first().map_or(ptr::null(), |s| s as *const _),
            rect_count,
            p_rects.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_regions: &[vk::ImageResolve],
    ) {
        let fp = self.fp_cmd_resolve_image.expect("vkCmdResolveImage is not loaded");
        let region_count = p_regions.len() as u32;
        (fp)(
            Some(command_buffer),
            Some(src_image),
            src_image_layout,
            Some(dst_image),
            dst_image_layout,
            region_count,
            p_regions.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        let fp = self.fp_cmd_set_event.expect("vkCmdSetEvent is not loaded");
        (fp)(Some(command_buffer), Some(event), stage_mask);
    }
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        let fp = self.fp_cmd_reset_event.expect("vkCmdResetEvent is not loaded");
        (fp)(Some(command_buffer), Some(event), stage_mask);
    }
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: vk::CommandBuffer,
        p_events: &[vk::Event],
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        p_memory_barriers: &[vk::MemoryBarrier],
        p_buffer_memory_barriers: &[vk::BufferMemoryBarrier],
        p_image_memory_barriers: &[vk::ImageMemoryBarrier],
    ) {
        let fp = self.fp_cmd_wait_events.expect("vkCmdWaitEvents is not loaded");
        let event_count = p_events.len() as u32;
        let memory_barrier_count = p_memory_barriers.len() as u32;
        let buffer_memory_barrier_count = p_buffer_memory_barriers.len() as u32;
        let image_memory_barrier_count = p_image_memory_barriers.len() as u32;
        (fp)(
            Some(command_buffer),
            event_count,
            p_events.first().map_or(ptr::null(), |s| s as *const _),
            src_stage_mask,
            dst_stage_mask,
            memory_barrier_count,
            p_memory_barriers.first().map_or(ptr::null(), |s| s as *const _),
            buffer_memory_barrier_count,
            p_buffer_memory_barriers.first().map_or(ptr::null(), |s| s as *const _),
            image_memory_barrier_count,
            p_image_memory_barriers.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: vk::CommandBuffer,
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        dependency_flags: vk::DependencyFlags,
        p_memory_barriers: &[vk::MemoryBarrier],
        p_buffer_memory_barriers: &[vk::BufferMemoryBarrier],
        p_image_memory_barriers: &[vk::ImageMemoryBarrier],
    ) {
        let fp = self
            .fp_cmd_pipeline_barrier
            .expect("vkCmdPipelineBarrier is not loaded");
        let memory_barrier_count = p_memory_barriers.len() as u32;
        let buffer_memory_barrier_count = p_buffer_memory_barriers.len() as u32;
        let image_memory_barrier_count = p_image_memory_barriers.len() as u32;
        (fp)(
            Some(command_buffer),
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barrier_count,
            p_memory_barriers.first().map_or(ptr::null(), |s| s as *const _),
            buffer_memory_barrier_count,
            p_buffer_memory_barriers.first().map_or(ptr::null(), |s| s as *const _),
            image_memory_barrier_count,
            p_image_memory_barriers.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        flags: vk::QueryControlFlags,
    ) {
        let fp = self.fp_cmd_begin_query.expect("vkCmdBeginQuery is not loaded");
        (fp)(Some(command_buffer), Some(query_pool), query, flags);
    }
    pub unsafe fn cmd_end_query(&self, command_buffer: vk::CommandBuffer, query_pool: vk::QueryPool, query: u32) {
        let fp = self.fp_cmd_end_query.expect("vkCmdEndQuery is not loaded");
        (fp)(Some(command_buffer), Some(query_pool), query);
    }
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_conditional_rendering_begin: &vk::ConditionalRenderingBeginInfoEXT,
    ) {
        let fp = self
            .fp_cmd_begin_conditional_rendering_ext
            .expect("vkCmdBeginConditionalRenderingEXT is not loaded");
        (fp)(Some(command_buffer), p_conditional_rendering_begin);
    }
    pub unsafe fn cmd_end_conditional_rendering_ext(&self, command_buffer: vk::CommandBuffer) {
        let fp = self
            .fp_cmd_end_conditional_rendering_ext
            .expect("vkCmdEndConditionalRenderingEXT is not loaded");
        (fp)(Some(command_buffer));
    }
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        let fp = self.fp_cmd_reset_query_pool.expect("vkCmdResetQueryPool is not loaded");
        (fp)(Some(command_buffer), Some(query_pool), first_query, query_count);
    }
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        let fp = self.fp_cmd_write_timestamp.expect("vkCmdWriteTimestamp is not loaded");
        (fp)(Some(command_buffer), pipeline_stage, Some(query_pool), query);
    }
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        stride: vk::DeviceSize,
        flags: vk::QueryResultFlags,
    ) {
        let fp = self
            .fp_cmd_copy_query_pool_results
            .expect("vkCmdCopyQueryPoolResults is not loaded");
        (fp)(
            Some(command_buffer),
            Some(query_pool),
            first_query,
            query_count,
            Some(dst_buffer),
            dst_offset,
            stride,
            flags,
        );
    }
    pub unsafe fn cmd_push_constants<T>(
        &self,
        command_buffer: vk::CommandBuffer,
        layout: vk::PipelineLayout,
        stage_flags: vk::ShaderStageFlags,
        offset: u32,
        p_values: &[T],
    ) {
        let fp = self.fp_cmd_push_constants.expect("vkCmdPushConstants is not loaded");
        let size = mem::size_of_val(p_values) as u32;
        (fp)(
            Some(command_buffer),
            Some(layout),
            stage_flags,
            offset,
            size,
            p_values.first().map_or(ptr::null(), |s| s as *const _) as *const _,
        );
    }
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: vk::CommandBuffer,
        p_render_pass_begin: &vk::RenderPassBeginInfo,
        contents: vk::SubpassContents,
    ) {
        let fp = self
            .fp_cmd_begin_render_pass
            .expect("vkCmdBeginRenderPass is not loaded");
        (fp)(Some(command_buffer), p_render_pass_begin, contents);
    }
    pub unsafe fn cmd_next_subpass(&self, command_buffer: vk::CommandBuffer, contents: vk::SubpassContents) {
        let fp = self.fp_cmd_next_subpass.expect("vkCmdNextSubpass is not loaded");
        (fp)(Some(command_buffer), contents);
    }
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: vk::CommandBuffer) {
        let fp = self.fp_cmd_end_render_pass.expect("vkCmdEndRenderPass is not loaded");
        (fp)(Some(command_buffer));
    }
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: vk::CommandBuffer,
        p_command_buffers: &[vk::CommandBuffer],
    ) {
        let fp = self
            .fp_cmd_execute_commands
            .expect("vkCmdExecuteCommands is not loaded");
        let command_buffer_count = p_command_buffers.len() as u32;
        (fp)(
            Some(command_buffer),
            command_buffer_count,
            p_command_buffers.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        p_create_infos: &[vk::SwapchainCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_swapchains: *mut vk::SwapchainKHR,
    ) -> Result<()> {
        let fp = self
            .fp_create_shared_swapchains_khr
            .expect("vkCreateSharedSwapchainsKHR is not loaded");
        let swapchain_count = p_create_infos.len() as u32;
        let v_err = (fp)(
            Some(self.handle),
            swapchain_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            p_swapchains,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_shared_swapchains_khr_to_vec(
        &self,
        p_create_infos: &[vk::SwapchainCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::SwapchainKHR>> {
        let fp = self
            .fp_create_shared_swapchains_khr
            .expect("vkCreateSharedSwapchainsKHR is not loaded");
        let swapchain_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(swapchain_count as usize);
        v.set_len(swapchain_count as usize);
        let v_err = (fp)(
            Some(self.handle),
            swapchain_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_shared_swapchains_khr_array<const N: usize>(
        &self,
        p_create_infos: &[vk::SwapchainCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<[vk::SwapchainKHR; N]> {
        let fp = self
            .fp_create_shared_swapchains_khr
            .expect("vkCreateSharedSwapchainsKHR is not loaded");
        let swapchain_count = p_create_infos.len() as u32;
        assert_eq!(swapchain_count, N as u32);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            swapchain_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr() as *mut _,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_shared_swapchains_khr_single(
        &self,
        p_create_infos: &vk::SwapchainCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SwapchainKHR> {
        let fp = self
            .fp_create_shared_swapchains_khr
            .expect("vkCreateSharedSwapchainsKHR is not loaded");
        let swapchain_count = 1;
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            swapchain_count,
            p_create_infos,
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_swapchain_khr(
        &self,
        p_create_info: &vk::SwapchainCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SwapchainKHR> {
        let fp = self
            .fp_create_swapchain_khr
            .expect("vkCreateSwapchainKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_swapchain_khr(
        &self,
        swapchain: Option<vk::SwapchainKHR>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_swapchain_khr
            .expect("vkDestroySwapchainKHR is not loaded");
        (fp)(Some(self.handle), swapchain, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_swapchain_images_khr_to_vec(&self, swapchain: vk::SwapchainKHR) -> Result<Vec<vk::Image>> {
        let fp = self
            .fp_get_swapchain_images_khr
            .expect("vkGetSwapchainImagesKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(self.handle), Some(swapchain), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(self.handle), Some(swapchain), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn acquire_next_image_khr(
        &self,
        swapchain: vk::SwapchainKHR,
        timeout: u64,
        semaphore: Option<vk::Semaphore>,
        fence: Option<vk::Fence>,
    ) -> Result<(vk::Result, u32)> {
        let fp = self
            .fp_acquire_next_image_khr
            .expect("vkAcquireNextImageKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            Some(swapchain),
            timeout,
            semaphore,
            fence,
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT | vk::Result::NOT_READY | vk::Result::SUBOPTIMAL_KHR => {
                Ok((err, res.assume_init()))
            }
            _ => Err(err),
        }
    }
    pub unsafe fn queue_present_khr(
        &self,
        queue: vk::Queue,
        p_present_info: &vk::PresentInfoKHR,
    ) -> Result<vk::Result> {
        let fp = self.fp_queue_present_khr.expect("vkQueuePresentKHR is not loaded");
        let err = (fp)(Some(queue), p_present_info);
        match err {
            vk::Result::SUCCESS | vk::Result::SUBOPTIMAL_KHR => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        p_name_info: &vk::DebugMarkerObjectNameInfoEXT,
    ) -> Result<()> {
        let fp = self
            .fp_debug_marker_set_object_name_ext
            .expect("vkDebugMarkerSetObjectNameEXT is not loaded");
        let err = (fp)(Some(self.handle), p_name_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn debug_marker_set_object_tag_ext(&self, p_tag_info: &vk::DebugMarkerObjectTagInfoEXT) -> Result<()> {
        let fp = self
            .fp_debug_marker_set_object_tag_ext
            .expect("vkDebugMarkerSetObjectTagEXT is not loaded");
        let err = (fp)(Some(self.handle), p_tag_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) {
        let fp = self
            .fp_cmd_debug_marker_begin_ext
            .expect("vkCmdDebugMarkerBeginEXT is not loaded");
        (fp)(Some(command_buffer), p_marker_info);
    }
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: vk::CommandBuffer) {
        let fp = self
            .fp_cmd_debug_marker_end_ext
            .expect("vkCmdDebugMarkerEndEXT is not loaded");
        (fp)(Some(command_buffer));
    }
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) {
        let fp = self
            .fp_cmd_debug_marker_insert_ext
            .expect("vkCmdDebugMarkerInsertEXT is not loaded");
        (fp)(Some(command_buffer), p_marker_info);
    }
    pub unsafe fn get_memory_win32_handle_nv(
        &self,
        memory: vk::DeviceMemory,
        handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<vk::HANDLE> {
        let fp = self
            .fp_get_memory_win32_handle_nv
            .expect("vkGetMemoryWin32HandleNV is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), Some(memory), handle_type, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        is_preprocessed: bool,
        p_generated_commands_info: &vk::GeneratedCommandsInfoNV,
    ) {
        let fp = self
            .fp_cmd_execute_generated_commands_nv
            .expect("vkCmdExecuteGeneratedCommandsNV is not loaded");
        (fp)(
            Some(command_buffer),
            if is_preprocessed { vk::TRUE } else { vk::FALSE },
            p_generated_commands_info,
        );
    }
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        p_generated_commands_info: &vk::GeneratedCommandsInfoNV,
    ) {
        let fp = self
            .fp_cmd_preprocess_generated_commands_nv
            .expect("vkCmdPreprocessGeneratedCommandsNV is not loaded");
        (fp)(Some(command_buffer), p_generated_commands_info);
    }
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
        group_index: u32,
    ) {
        let fp = self
            .fp_cmd_bind_pipeline_shader_group_nv
            .expect("vkCmdBindPipelineShaderGroupNV is not loaded");
        (fp)(Some(command_buffer), pipeline_bind_point, Some(pipeline), group_index);
    }
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        p_info: &vk::GeneratedCommandsMemoryRequirementsInfoNV,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        let fp = self
            .fp_get_generated_commands_memory_requirements_nv
            .expect("vkGetGeneratedCommandsMemoryRequirementsNV is not loaded");
        (fp)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn create_indirect_commands_layout_nv(
        &self,
        p_create_info: &vk::IndirectCommandsLayoutCreateInfoNV,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::IndirectCommandsLayoutNV> {
        let fp = self
            .fp_create_indirect_commands_layout_nv
            .expect("vkCreateIndirectCommandsLayoutNV is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        indirect_commands_layout: Option<vk::IndirectCommandsLayoutNV>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_indirect_commands_layout_nv
            .expect("vkDestroyIndirectCommandsLayoutNV is not loaded");
        (fp)(
            Some(self.handle),
            indirect_commands_layout,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: u32,
        p_descriptor_writes: &[vk::WriteDescriptorSet],
    ) {
        let fp = self
            .fp_cmd_push_descriptor_set_khr
            .expect("vkCmdPushDescriptorSetKHR is not loaded");
        let descriptor_write_count = p_descriptor_writes.len() as u32;
        (fp)(
            Some(command_buffer),
            pipeline_bind_point,
            Some(layout),
            set,
            descriptor_write_count,
            p_descriptor_writes.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn trim_command_pool(&self, command_pool: vk::CommandPool, flags: vk::CommandPoolTrimFlags) {
        let fp = self.fp_trim_command_pool.expect("vkTrimCommandPool is not loaded");
        (fp)(Some(self.handle), Some(command_pool), flags);
    }
    pub unsafe fn trim_command_pool_khr(&self, command_pool: vk::CommandPool, flags: vk::CommandPoolTrimFlags) {
        let fp = self.fp_trim_command_pool.expect("vkTrimCommandPoolKHR is not loaded");
        (fp)(Some(self.handle), Some(command_pool), flags);
    }
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &vk::MemoryGetWin32HandleInfoKHR,
    ) -> Result<vk::HANDLE> {
        let fp = self
            .fp_get_memory_win32_handle_khr
            .expect("vkGetMemoryWin32HandleKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_get_win32_handle_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        handle: vk::HANDLE,
        p_memory_win32_handle_properties: &mut vk::MemoryWin32HandlePropertiesKHR,
    ) -> Result<()> {
        let fp = self
            .fp_get_memory_win32_handle_properties_khr
            .expect("vkGetMemoryWin32HandlePropertiesKHR is not loaded");
        let err = (fp)(Some(self.handle), handle_type, handle, p_memory_win32_handle_properties);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_memory_fd_khr(&self, p_get_fd_info: &vk::MemoryGetFdInfoKHR) -> Result<c_int> {
        let fp = self.fp_get_memory_fd_khr.expect("vkGetMemoryFdKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_get_fd_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        fd: c_int,
        p_memory_fd_properties: &mut vk::MemoryFdPropertiesKHR,
    ) -> Result<()> {
        let fp = self
            .fp_get_memory_fd_properties_khr
            .expect("vkGetMemoryFdPropertiesKHR is not loaded");
        let err = (fp)(Some(self.handle), handle_type, fd, p_memory_fd_properties);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_memory_zircon_handle_fuchsia(
        &self,
        p_get_zircon_handle_info: &vk::MemoryGetZirconHandleInfoFUCHSIA,
    ) -> Result<vk::zx_handle_t> {
        let fp = self
            .fp_get_memory_zircon_handle_fuchsia
            .expect("vkGetMemoryZirconHandleFUCHSIA is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_get_zircon_handle_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        zircon_handle: vk::zx_handle_t,
        p_memory_zircon_handle_properties: &mut vk::MemoryZirconHandlePropertiesFUCHSIA,
    ) -> Result<()> {
        let fp = self
            .fp_get_memory_zircon_handle_properties_fuchsia
            .expect("vkGetMemoryZirconHandlePropertiesFUCHSIA is not loaded");
        let err = (fp)(
            Some(self.handle),
            handle_type,
            zircon_handle,
            p_memory_zircon_handle_properties,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &vk::SemaphoreGetWin32HandleInfoKHR,
    ) -> Result<vk::HANDLE> {
        let fp = self
            .fp_get_semaphore_win32_handle_khr
            .expect("vkGetSemaphoreWin32HandleKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_get_win32_handle_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        p_import_semaphore_win32_handle_info: &vk::ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result<()> {
        let fp = self
            .fp_import_semaphore_win32_handle_khr
            .expect("vkImportSemaphoreWin32HandleKHR is not loaded");
        let err = (fp)(Some(self.handle), p_import_semaphore_win32_handle_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_semaphore_fd_khr(&self, p_get_fd_info: &vk::SemaphoreGetFdInfoKHR) -> Result<c_int> {
        let fp = self.fp_get_semaphore_fd_khr.expect("vkGetSemaphoreFdKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_get_fd_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        p_import_semaphore_fd_info: &vk::ImportSemaphoreFdInfoKHR,
    ) -> Result<()> {
        let fp = self
            .fp_import_semaphore_fd_khr
            .expect("vkImportSemaphoreFdKHR is not loaded");
        let err = (fp)(Some(self.handle), p_import_semaphore_fd_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(
        &self,
        p_get_zircon_handle_info: &vk::SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> Result<vk::zx_handle_t> {
        let fp = self
            .fp_get_semaphore_zircon_handle_fuchsia
            .expect("vkGetSemaphoreZirconHandleFUCHSIA is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_get_zircon_handle_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(
        &self,
        p_import_semaphore_zircon_handle_info: &vk::ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> Result<()> {
        let fp = self
            .fp_import_semaphore_zircon_handle_fuchsia
            .expect("vkImportSemaphoreZirconHandleFUCHSIA is not loaded");
        let err = (fp)(Some(self.handle), p_import_semaphore_zircon_handle_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_fence_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &vk::FenceGetWin32HandleInfoKHR,
    ) -> Result<vk::HANDLE> {
        let fp = self
            .fp_get_fence_win32_handle_khr
            .expect("vkGetFenceWin32HandleKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_get_win32_handle_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        p_import_fence_win32_handle_info: &vk::ImportFenceWin32HandleInfoKHR,
    ) -> Result<()> {
        let fp = self
            .fp_import_fence_win32_handle_khr
            .expect("vkImportFenceWin32HandleKHR is not loaded");
        let err = (fp)(Some(self.handle), p_import_fence_win32_handle_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_fence_fd_khr(&self, p_get_fd_info: &vk::FenceGetFdInfoKHR) -> Result<c_int> {
        let fp = self.fp_get_fence_fd_khr.expect("vkGetFenceFdKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_get_fd_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn import_fence_fd_khr(&self, p_import_fence_fd_info: &vk::ImportFenceFdInfoKHR) -> Result<()> {
        let fp = self.fp_import_fence_fd_khr.expect("vkImportFenceFdKHR is not loaded");
        let err = (fp)(Some(self.handle), p_import_fence_fd_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn acquire_winrt_display_nv(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> Result<()> {
        let fp = self
            .fp_acquire_winrt_display_nv
            .expect("vkAcquireWinrtDisplayNV is not loaded");
        let err = (fp)(Some(physical_device), Some(display));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_winrt_display_nv(
        &self,
        physical_device: vk::PhysicalDevice,
        device_relative_id: u32,
    ) -> Result<vk::DisplayKHR> {
        let fp = self.fp_get_winrt_display_nv.expect("vkGetWinrtDisplayNV is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(physical_device), device_relative_id, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn display_power_control_ext(
        &self,
        display: vk::DisplayKHR,
        p_display_power_info: &vk::DisplayPowerInfoEXT,
    ) -> Result<()> {
        let fp = self
            .fp_display_power_control_ext
            .expect("vkDisplayPowerControlEXT is not loaded");
        let err = (fp)(Some(self.handle), Some(display), p_display_power_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn register_device_event_ext(
        &self,
        p_device_event_info: &vk::DeviceEventInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Fence> {
        let fp = self
            .fp_register_device_event_ext
            .expect("vkRegisterDeviceEventEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_device_event_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn register_display_event_ext(
        &self,
        display: vk::DisplayKHR,
        p_display_event_info: &vk::DisplayEventInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Fence> {
        let fp = self
            .fp_register_display_event_ext
            .expect("vkRegisterDisplayEventEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            Some(display),
            p_display_event_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        swapchain: vk::SwapchainKHR,
        counter: vk::SurfaceCounterFlagsEXT,
    ) -> Result<u64> {
        let fp = self
            .fp_get_swapchain_counter_ext
            .expect("vkGetSwapchainCounterEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), Some(swapchain), counter, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> vk::PeerMemoryFeatureFlags {
        let fp = self
            .fp_get_device_group_peer_memory_features
            .expect("vkGetDeviceGroupPeerMemoryFeatures is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(
            Some(self.handle),
            heap_index,
            local_device_index,
            remote_device_index,
            res.as_mut_ptr(),
        );
        res.assume_init()
    }
    pub unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> vk::PeerMemoryFeatureFlags {
        let fp = self
            .fp_get_device_group_peer_memory_features
            .expect("vkGetDeviceGroupPeerMemoryFeaturesKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(
            Some(self.handle),
            heap_index,
            local_device_index,
            remote_device_index,
            res.as_mut_ptr(),
        );
        res.assume_init()
    }
    pub unsafe fn bind_buffer_memory2(&self, p_bind_infos: &[vk::BindBufferMemoryInfo]) -> Result<()> {
        let fp = self.fp_bind_buffer_memory2.expect("vkBindBufferMemory2 is not loaded");
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (fp)(
            Some(self.handle),
            bind_info_count,
            p_bind_infos.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn bind_buffer_memory2_khr(&self, p_bind_infos: &[vk::BindBufferMemoryInfo]) -> Result<()> {
        let fp = self
            .fp_bind_buffer_memory2
            .expect("vkBindBufferMemory2KHR is not loaded");
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (fp)(
            Some(self.handle),
            bind_info_count,
            p_bind_infos.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn bind_image_memory2(&self, p_bind_infos: &[vk::BindImageMemoryInfo]) -> Result<()> {
        let fp = self.fp_bind_image_memory2.expect("vkBindImageMemory2 is not loaded");
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (fp)(
            Some(self.handle),
            bind_info_count,
            p_bind_infos.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn bind_image_memory2_khr(&self, p_bind_infos: &[vk::BindImageMemoryInfo]) -> Result<()> {
        let fp = self.fp_bind_image_memory2.expect("vkBindImageMemory2KHR is not loaded");
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (fp)(
            Some(self.handle),
            bind_info_count,
            p_bind_infos.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: vk::CommandBuffer, device_mask: u32) {
        let fp = self.fp_cmd_set_device_mask.expect("vkCmdSetDeviceMask is not loaded");
        (fp)(Some(command_buffer), device_mask);
    }
    pub unsafe fn cmd_set_device_mask_khr(&self, command_buffer: vk::CommandBuffer, device_mask: u32) {
        let fp = self
            .fp_cmd_set_device_mask
            .expect("vkCmdSetDeviceMaskKHR is not loaded");
        (fp)(Some(command_buffer), device_mask);
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        p_device_group_present_capabilities: &mut vk::DeviceGroupPresentCapabilitiesKHR,
    ) -> Result<()> {
        let fp = self
            .fp_get_device_group_present_capabilities_khr
            .expect("vkGetDeviceGroupPresentCapabilitiesKHR is not loaded");
        let err = (fp)(Some(self.handle), p_device_group_present_capabilities);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        surface: vk::SurfaceKHR,
    ) -> Result<vk::DeviceGroupPresentModeFlagsKHR> {
        let fp = self
            .fp_get_device_group_surface_present_modes_khr
            .expect("vkGetDeviceGroupSurfacePresentModesKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), Some(surface), res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        p_acquire_info: &vk::AcquireNextImageInfoKHR,
    ) -> Result<(vk::Result, u32)> {
        let fp = self
            .fp_acquire_next_image2_khr
            .expect("vkAcquireNextImage2KHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_acquire_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT | vk::Result::NOT_READY | vk::Result::SUBOPTIMAL_KHR => {
                Ok((err, res.assume_init()))
            }
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        let fp = self.fp_cmd_dispatch_base.expect("vkCmdDispatchBase is not loaded");
        (fp)(
            Some(command_buffer),
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }
    pub unsafe fn cmd_dispatch_base_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        let fp = self.fp_cmd_dispatch_base.expect("vkCmdDispatchBaseKHR is not loaded");
        (fp)(
            Some(command_buffer),
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }
    pub unsafe fn get_physical_device_present_rectangles_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Vec<vk::Rect2D>> {
        let fp = self
            .fp_get_physical_device_present_rectangles_khr
            .expect("vkGetPhysicalDevicePresentRectanglesKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), Some(surface), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), Some(surface), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_descriptor_update_template(
        &self,
        p_create_info: &vk::DescriptorUpdateTemplateCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorUpdateTemplate> {
        let fp = self
            .fp_create_descriptor_update_template
            .expect("vkCreateDescriptorUpdateTemplate is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_descriptor_update_template_khr(
        &self,
        p_create_info: &vk::DescriptorUpdateTemplateCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorUpdateTemplate> {
        let fp = self
            .fp_create_descriptor_update_template
            .expect("vkCreateDescriptorUpdateTemplateKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_descriptor_update_template
            .expect("vkDestroyDescriptorUpdateTemplate is not loaded");
        (fp)(
            Some(self.handle),
            descriptor_update_template,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_descriptor_update_template
            .expect("vkDestroyDescriptorUpdateTemplateKHR is not loaded");
        (fp)(
            Some(self.handle),
            descriptor_update_template,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        let fp = self
            .fp_update_descriptor_set_with_template
            .expect("vkUpdateDescriptorSetWithTemplate is not loaded");
        (fp)(
            Some(self.handle),
            Some(descriptor_set),
            Some(descriptor_update_template),
            p_data,
        );
    }
    pub unsafe fn update_descriptor_set_with_template_khr(
        &self,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        let fp = self
            .fp_update_descriptor_set_with_template
            .expect("vkUpdateDescriptorSetWithTemplateKHR is not loaded");
        (fp)(
            Some(self.handle),
            Some(descriptor_set),
            Some(descriptor_update_template),
            p_data,
        );
    }
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        let fp = self
            .fp_cmd_push_descriptor_set_with_template_khr
            .expect("vkCmdPushDescriptorSetWithTemplateKHR is not loaded");
        (fp)(
            Some(command_buffer),
            Some(descriptor_update_template),
            Some(layout),
            set,
            p_data,
        );
    }
    pub unsafe fn set_hdr_metadata_ext(&self, p_swapchains: &[vk::SwapchainKHR], p_metadata: &[vk::HdrMetadataEXT]) {
        let fp = self.fp_set_hdr_metadata_ext.expect("vkSetHdrMetadataEXT is not loaded");
        let swapchain_count = p_swapchains.len() as u32;
        assert_eq!(swapchain_count, p_metadata.len() as u32);
        (fp)(
            Some(self.handle),
            swapchain_count,
            p_swapchains.first().map_or(ptr::null(), |s| s as *const _),
            p_metadata.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn get_swapchain_status_khr(&self, swapchain: vk::SwapchainKHR) -> Result<vk::Result> {
        let fp = self
            .fp_get_swapchain_status_khr
            .expect("vkGetSwapchainStatusKHR is not loaded");
        let err = (fp)(Some(self.handle), Some(swapchain));
        match err {
            vk::Result::SUCCESS | vk::Result::SUBOPTIMAL_KHR => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> Result<vk::RefreshCycleDurationGOOGLE> {
        let fp = self
            .fp_get_refresh_cycle_duration_google
            .expect("vkGetRefreshCycleDurationGOOGLE is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), Some(swapchain), res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_past_presentation_timing_google_to_vec(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> Result<Vec<vk::PastPresentationTimingGOOGLE>> {
        let fp = self
            .fp_get_past_presentation_timing_google
            .expect("vkGetPastPresentationTimingGOOGLE is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(self.handle), Some(swapchain), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(self.handle), Some(swapchain), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_viewport_w_scalings: &[vk::ViewportWScalingNV],
    ) {
        let fp = self
            .fp_cmd_set_viewport_w_scaling_nv
            .expect("vkCmdSetViewportWScalingNV is not loaded");
        let viewport_count = p_viewport_w_scalings.len() as u32;
        (fp)(
            Some(command_buffer),
            first_viewport,
            viewport_count,
            p_viewport_w_scalings.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_discard_rectangle: u32,
        p_discard_rectangles: &[vk::Rect2D],
    ) {
        let fp = self
            .fp_cmd_set_discard_rectangle_ext
            .expect("vkCmdSetDiscardRectangleEXT is not loaded");
        let discard_rectangle_count = p_discard_rectangles.len() as u32;
        (fp)(
            Some(command_buffer),
            first_discard_rectangle,
            discard_rectangle_count,
            p_discard_rectangles.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_sample_locations_info: &vk::SampleLocationsInfoEXT,
    ) {
        let fp = self
            .fp_cmd_set_sample_locations_ext
            .expect("vkCmdSetSampleLocationsEXT is not loaded");
        (fp)(Some(command_buffer), p_sample_locations_info);
    }
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        samples: vk::SampleCountFlags,
        p_multisample_properties: &mut vk::MultisamplePropertiesEXT,
    ) {
        let fp = self
            .fp_get_physical_device_multisample_properties_ext
            .expect("vkGetPhysicalDeviceMultisamplePropertiesEXT is not loaded");
        (fp)(Some(physical_device), samples, p_multisample_properties);
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        p_info: &vk::BufferMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        let fp = self
            .fp_get_buffer_memory_requirements2
            .expect("vkGetBufferMemoryRequirements2 is not loaded");
        (fp)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_buffer_memory_requirements2_khr(
        &self,
        p_info: &vk::BufferMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        let fp = self
            .fp_get_buffer_memory_requirements2
            .expect("vkGetBufferMemoryRequirements2KHR is not loaded");
        (fp)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        p_info: &vk::ImageMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        let fp = self
            .fp_get_image_memory_requirements2
            .expect("vkGetImageMemoryRequirements2 is not loaded");
        (fp)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_image_memory_requirements2_khr(
        &self,
        p_info: &vk::ImageMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        let fp = self
            .fp_get_image_memory_requirements2
            .expect("vkGetImageMemoryRequirements2KHR is not loaded");
        (fp)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_image_sparse_memory_requirements2_to_vec(
        &self,
        p_info: &vk::ImageSparseMemoryRequirementsInfo2,
    ) -> Vec<vk::SparseImageMemoryRequirements2> {
        let fp = self
            .fp_get_image_sparse_memory_requirements2
            .expect("vkGetImageSparseMemoryRequirements2 is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), p_info, len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(self.handle), p_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn get_image_sparse_memory_requirements2_khr_to_vec(
        &self,
        p_info: &vk::ImageSparseMemoryRequirementsInfo2,
    ) -> Vec<vk::SparseImageMemoryRequirements2> {
        let fp = self
            .fp_get_image_sparse_memory_requirements2
            .expect("vkGetImageSparseMemoryRequirements2KHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), p_info, len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(self.handle), p_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        p_create_info: &vk::SamplerYcbcrConversionCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SamplerYcbcrConversion> {
        let fp = self
            .fp_create_sampler_ycbcr_conversion
            .expect("vkCreateSamplerYcbcrConversion is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        p_create_info: &vk::SamplerYcbcrConversionCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SamplerYcbcrConversion> {
        let fp = self
            .fp_create_sampler_ycbcr_conversion
            .expect("vkCreateSamplerYcbcrConversionKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_sampler_ycbcr_conversion
            .expect("vkDestroySamplerYcbcrConversion is not loaded");
        (fp)(
            Some(self.handle),
            ycbcr_conversion,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_sampler_ycbcr_conversion
            .expect("vkDestroySamplerYcbcrConversionKHR is not loaded");
        (fp)(
            Some(self.handle),
            ycbcr_conversion,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_device_queue2(&self, p_queue_info: &vk::DeviceQueueInfo2) -> vk::Queue {
        let fp = self.fp_get_device_queue2.expect("vkGetDeviceQueue2 is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), p_queue_info, res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn create_validation_cache_ext(
        &self,
        p_create_info: &vk::ValidationCacheCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ValidationCacheEXT> {
        let fp = self
            .fp_create_validation_cache_ext
            .expect("vkCreateValidationCacheEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        validation_cache: Option<vk::ValidationCacheEXT>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_validation_cache_ext
            .expect("vkDestroyValidationCacheEXT is not loaded");
        (fp)(
            Some(self.handle),
            validation_cache,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        validation_cache: vk::ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result<vk::Result> {
        let fp = self
            .fp_get_validation_cache_data_ext
            .expect("vkGetValidationCacheDataEXT is not loaded");
        let err = (fp)(Some(self.handle), Some(validation_cache), p_data_size, p_data);
        match err {
            vk::Result::SUCCESS | vk::Result::INCOMPLETE => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn merge_validation_caches_ext(
        &self,
        dst_cache: vk::ValidationCacheEXT,
        p_src_caches: &[vk::ValidationCacheEXT],
    ) -> Result<()> {
        let fp = self
            .fp_merge_validation_caches_ext
            .expect("vkMergeValidationCachesEXT is not loaded");
        let src_cache_count = p_src_caches.len() as u32;
        let err = (fp)(
            Some(self.handle),
            Some(dst_cache),
            src_cache_count,
            p_src_caches.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_support: &mut vk::DescriptorSetLayoutSupport,
    ) {
        let fp = self
            .fp_get_descriptor_set_layout_support
            .expect("vkGetDescriptorSetLayoutSupport is not loaded");
        (fp)(Some(self.handle), p_create_info, p_support);
    }
    pub unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_support: &mut vk::DescriptorSetLayoutSupport,
    ) {
        let fp = self
            .fp_get_descriptor_set_layout_support
            .expect("vkGetDescriptorSetLayoutSupportKHR is not loaded");
        (fp)(Some(self.handle), p_create_info, p_support);
    }
    pub unsafe fn get_shader_info_amd(
        &self,
        pipeline: vk::Pipeline,
        shader_stage: vk::ShaderStageFlags,
        info_type: vk::ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> Result<vk::Result> {
        let fp = self.fp_get_shader_info_amd.expect("vkGetShaderInfoAMD is not loaded");
        let err = (fp)(
            Some(self.handle),
            Some(pipeline),
            shader_stage,
            info_type,
            p_info_size,
            p_info,
        );
        match err {
            vk::Result::SUCCESS | vk::Result::INCOMPLETE => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn set_local_dimming_amd(&self, swap_chain: vk::SwapchainKHR, local_dimming_enable: bool) {
        let fp = self
            .fp_set_local_dimming_amd
            .expect("vkSetLocalDimmingAMD is not loaded");
        (fp)(
            Some(self.handle),
            Some(swap_chain),
            if local_dimming_enable { vk::TRUE } else { vk::FALSE },
        );
    }
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::TimeDomainEXT>> {
        let fp = self
            .fp_get_physical_device_calibrateable_time_domains_ext
            .expect("vkGetPhysicalDeviceCalibrateableTimeDomainsEXT is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_calibrated_timestamps_ext(
        &self,
        p_timestamp_infos: &[vk::CalibratedTimestampInfoEXT],
        p_timestamps: *mut u64,
        p_max_deviation: &mut u64,
    ) -> Result<()> {
        let fp = self
            .fp_get_calibrated_timestamps_ext
            .expect("vkGetCalibratedTimestampsEXT is not loaded");
        let timestamp_count = p_timestamp_infos.len() as u32;
        let v_err = (fp)(
            Some(self.handle),
            timestamp_count,
            p_timestamp_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_timestamps,
            p_max_deviation,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_calibrated_timestamps_ext_to_vec(
        &self,
        p_timestamp_infos: &[vk::CalibratedTimestampInfoEXT],
        p_max_deviation: &mut u64,
    ) -> Result<Vec<u64>> {
        let fp = self
            .fp_get_calibrated_timestamps_ext
            .expect("vkGetCalibratedTimestampsEXT is not loaded");
        let timestamp_count = p_timestamp_infos.len() as u32;
        let mut v = Vec::with_capacity(timestamp_count as usize);
        v.set_len(timestamp_count as usize);
        let v_err = (fp)(
            Some(self.handle),
            timestamp_count,
            p_timestamp_infos.first().map_or(ptr::null(), |s| s as *const _),
            v.as_mut_ptr(),
            p_max_deviation,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_calibrated_timestamps_ext_array<const N: usize>(
        &self,
        p_timestamp_infos: &[vk::CalibratedTimestampInfoEXT],
        p_max_deviation: &mut u64,
    ) -> Result<[u64; N]> {
        let fp = self
            .fp_get_calibrated_timestamps_ext
            .expect("vkGetCalibratedTimestampsEXT is not loaded");
        let timestamp_count = p_timestamp_infos.len() as u32;
        assert_eq!(timestamp_count, N as u32);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            timestamp_count,
            p_timestamp_infos.first().map_or(ptr::null(), |s| s as *const _),
            v.as_mut_ptr() as *mut _,
            p_max_deviation,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_calibrated_timestamps_ext_single(
        &self,
        p_timestamp_infos: &vk::CalibratedTimestampInfoEXT,
        p_max_deviation: &mut u64,
    ) -> Result<u64> {
        let fp = self
            .fp_get_calibrated_timestamps_ext
            .expect("vkGetCalibratedTimestampsEXT is not loaded");
        let timestamp_count = 1;
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            timestamp_count,
            p_timestamp_infos,
            v.as_mut_ptr(),
            p_max_deviation,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: &mut vk::MemoryHostPointerPropertiesEXT,
    ) -> Result<()> {
        let fp = self
            .fp_get_memory_host_pointer_properties_ext
            .expect("vkGetMemoryHostPointerPropertiesEXT is not loaded");
        let err = (fp)(
            Some(self.handle),
            handle_type,
            p_host_pointer,
            p_memory_host_pointer_properties,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        marker: u32,
    ) {
        let fp = self
            .fp_cmd_write_buffer_marker_amd
            .expect("vkCmdWriteBufferMarkerAMD is not loaded");
        (fp)(
            Some(command_buffer),
            pipeline_stage,
            Some(dst_buffer),
            dst_offset,
            marker,
        );
    }
    pub unsafe fn create_render_pass2(
        &self,
        p_create_info: &vk::RenderPassCreateInfo2,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::RenderPass> {
        let fp = self.fp_create_render_pass2.expect("vkCreateRenderPass2 is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_render_pass2_khr(
        &self,
        p_create_info: &vk::RenderPassCreateInfo2,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::RenderPass> {
        let fp = self
            .fp_create_render_pass2
            .expect("vkCreateRenderPass2KHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        p_render_pass_begin: &vk::RenderPassBeginInfo,
        p_subpass_begin_info: &vk::SubpassBeginInfo,
    ) {
        let fp = self
            .fp_cmd_begin_render_pass2
            .expect("vkCmdBeginRenderPass2 is not loaded");
        (fp)(Some(command_buffer), p_render_pass_begin, p_subpass_begin_info);
    }
    pub unsafe fn cmd_begin_render_pass2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_render_pass_begin: &vk::RenderPassBeginInfo,
        p_subpass_begin_info: &vk::SubpassBeginInfo,
    ) {
        let fp = self
            .fp_cmd_begin_render_pass2
            .expect("vkCmdBeginRenderPass2KHR is not loaded");
        (fp)(Some(command_buffer), p_render_pass_begin, p_subpass_begin_info);
    }
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: vk::CommandBuffer,
        p_subpass_begin_info: &vk::SubpassBeginInfo,
        p_subpass_end_info: &vk::SubpassEndInfo,
    ) {
        let fp = self.fp_cmd_next_subpass2.expect("vkCmdNextSubpass2 is not loaded");
        (fp)(Some(command_buffer), p_subpass_begin_info, p_subpass_end_info);
    }
    pub unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_subpass_begin_info: &vk::SubpassBeginInfo,
        p_subpass_end_info: &vk::SubpassEndInfo,
    ) {
        let fp = self.fp_cmd_next_subpass2.expect("vkCmdNextSubpass2KHR is not loaded");
        (fp)(Some(command_buffer), p_subpass_begin_info, p_subpass_end_info);
    }
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        p_subpass_end_info: &vk::SubpassEndInfo,
    ) {
        let fp = self.fp_cmd_end_render_pass2.expect("vkCmdEndRenderPass2 is not loaded");
        (fp)(Some(command_buffer), p_subpass_end_info);
    }
    pub unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_subpass_end_info: &vk::SubpassEndInfo,
    ) {
        let fp = self
            .fp_cmd_end_render_pass2
            .expect("vkCmdEndRenderPass2KHR is not loaded");
        (fp)(Some(command_buffer), p_subpass_end_info);
    }
    pub unsafe fn get_semaphore_counter_value(&self, semaphore: vk::Semaphore) -> Result<u64> {
        let fp = self
            .fp_get_semaphore_counter_value
            .expect("vkGetSemaphoreCounterValue is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), Some(semaphore), res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_semaphore_counter_value_khr(&self, semaphore: vk::Semaphore) -> Result<u64> {
        let fp = self
            .fp_get_semaphore_counter_value
            .expect("vkGetSemaphoreCounterValueKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), Some(semaphore), res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn wait_semaphores(&self, p_wait_info: &vk::SemaphoreWaitInfo, timeout: u64) -> Result<vk::Result> {
        let fp = self.fp_wait_semaphores.expect("vkWaitSemaphores is not loaded");
        let err = (fp)(Some(self.handle), p_wait_info, timeout);
        match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn wait_semaphores_khr(&self, p_wait_info: &vk::SemaphoreWaitInfo, timeout: u64) -> Result<vk::Result> {
        let fp = self.fp_wait_semaphores.expect("vkWaitSemaphoresKHR is not loaded");
        let err = (fp)(Some(self.handle), p_wait_info, timeout);
        match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn signal_semaphore(&self, p_signal_info: &vk::SemaphoreSignalInfo) -> Result<()> {
        let fp = self.fp_signal_semaphore.expect("vkSignalSemaphore is not loaded");
        let err = (fp)(Some(self.handle), p_signal_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn signal_semaphore_khr(&self, p_signal_info: &vk::SemaphoreSignalInfo) -> Result<()> {
        let fp = self.fp_signal_semaphore.expect("vkSignalSemaphoreKHR is not loaded");
        let err = (fp)(Some(self.handle), p_signal_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        buffer: &vk::AHardwareBuffer,
        p_properties: &mut vk::AndroidHardwareBufferPropertiesANDROID,
    ) -> Result<()> {
        let fp = self
            .fp_get_android_hardware_buffer_properties_android
            .expect("vkGetAndroidHardwareBufferPropertiesANDROID is not loaded");
        let err = (fp)(Some(self.handle), buffer, p_properties);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        p_info: &vk::MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> Result<*mut vk::AHardwareBuffer> {
        let fp = self
            .fp_get_memory_android_hardware_buffer_android
            .expect("vkGetMemoryAndroidHardwareBufferANDROID is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_indirect_count
            .expect("vkCmdDrawIndirectCount is not loaded");
        (fp)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_draw_indirect_count_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_indirect_count
            .expect("vkCmdDrawIndirectCountKHR is not loaded");
        (fp)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_draw_indirect_count_amd(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_indirect_count
            .expect("vkCmdDrawIndirectCountAMD is not loaded");
        (fp)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_indexed_indirect_count
            .expect("vkCmdDrawIndexedIndirectCount is not loaded");
        (fp)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_indexed_indirect_count
            .expect("vkCmdDrawIndexedIndirectCountKHR is not loaded");
        (fp)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_indexed_indirect_count
            .expect("vkCmdDrawIndexedIndirectCountAMD is not loaded");
        (fp)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_set_checkpoint_nv(&self, command_buffer: vk::CommandBuffer, p_checkpoint_marker: *const c_void) {
        let fp = self
            .fp_cmd_set_checkpoint_nv
            .expect("vkCmdSetCheckpointNV is not loaded");
        (fp)(Some(command_buffer), p_checkpoint_marker);
    }
    pub unsafe fn get_queue_checkpoint_data_nv_to_vec(&self, queue: vk::Queue) -> Vec<vk::CheckpointDataNV> {
        let fp = self
            .fp_get_queue_checkpoint_data_nv
            .expect("vkGetQueueCheckpointDataNV is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(queue), len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(queue), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        p_buffers: &[vk::Buffer],
        p_offsets: &[vk::DeviceSize],
        p_sizes: Option<&[vk::DeviceSize]>,
    ) {
        let fp = self
            .fp_cmd_bind_transform_feedback_buffers_ext
            .expect("vkCmdBindTransformFeedbackBuffersEXT is not loaded");
        let binding_count = p_buffers.len() as u32;
        assert_eq!(binding_count, p_offsets.len() as u32);
        if let Some(s) = p_sizes {
            assert_eq!(binding_count, s.len() as u32);
        }
        (fp)(
            Some(command_buffer),
            first_binding,
            binding_count,
            p_buffers.first().map_or(ptr::null(), |s| s as *const _),
            p_offsets.first().map_or(ptr::null(), |s| s as *const _),
            p_sizes.and_then(|s| s.first()).map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_begin_transform_feedback_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        p_counter_buffers: &[vk::Buffer],
        p_counter_buffer_offsets: Option<&[vk::DeviceSize]>,
    ) {
        let fp = self
            .fp_cmd_begin_transform_feedback_ext
            .expect("vkCmdBeginTransformFeedbackEXT is not loaded");
        let counter_buffer_count = p_counter_buffers.len() as u32;
        if let Some(s) = p_counter_buffer_offsets {
            assert_eq!(counter_buffer_count, s.len() as u32);
        }
        (fp)(
            Some(command_buffer),
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers.first().map_or(ptr::null(), |s| s as *const _),
            p_counter_buffer_offsets
                .and_then(|s| s.first())
                .map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        p_counter_buffers: &[vk::Buffer],
        p_counter_buffer_offsets: Option<&[vk::DeviceSize]>,
    ) {
        let fp = self
            .fp_cmd_end_transform_feedback_ext
            .expect("vkCmdEndTransformFeedbackEXT is not loaded");
        let counter_buffer_count = p_counter_buffers.len() as u32;
        if let Some(s) = p_counter_buffer_offsets {
            assert_eq!(counter_buffer_count, s.len() as u32);
        }
        (fp)(
            Some(command_buffer),
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers.first().map_or(ptr::null(), |s| s as *const _),
            p_counter_buffer_offsets
                .and_then(|s| s.first())
                .map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_begin_query_indexed_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        flags: vk::QueryControlFlags,
        index: u32,
    ) {
        let fp = self
            .fp_cmd_begin_query_indexed_ext
            .expect("vkCmdBeginQueryIndexedEXT is not loaded");
        (fp)(Some(command_buffer), Some(query_pool), query, flags, index);
    }
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        index: u32,
    ) {
        let fp = self
            .fp_cmd_end_query_indexed_ext
            .expect("vkCmdEndQueryIndexedEXT is not loaded");
        (fp)(Some(command_buffer), Some(query_pool), query, index);
    }
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: vk::Buffer,
        counter_buffer_offset: vk::DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_indirect_byte_count_ext
            .expect("vkCmdDrawIndirectByteCountEXT is not loaded");
        (fp)(
            Some(command_buffer),
            instance_count,
            first_instance,
            Some(counter_buffer),
            counter_buffer_offset,
            counter_offset,
            vertex_stride,
        );
    }
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_exclusive_scissor: u32,
        p_exclusive_scissors: &[vk::Rect2D],
    ) {
        let fp = self
            .fp_cmd_set_exclusive_scissor_nv
            .expect("vkCmdSetExclusiveScissorNV is not loaded");
        let exclusive_scissor_count = p_exclusive_scissors.len() as u32;
        (fp)(
            Some(command_buffer),
            first_exclusive_scissor,
            exclusive_scissor_count,
            p_exclusive_scissors.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        image_view: Option<vk::ImageView>,
        image_layout: vk::ImageLayout,
    ) {
        let fp = self
            .fp_cmd_bind_shading_rate_image_nv
            .expect("vkCmdBindShadingRateImageNV is not loaded");
        (fp)(Some(command_buffer), image_view, image_layout);
    }
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_shading_rate_palettes: &[vk::ShadingRatePaletteNV],
    ) {
        let fp = self
            .fp_cmd_set_viewport_shading_rate_palette_nv
            .expect("vkCmdSetViewportShadingRatePaletteNV is not loaded");
        let viewport_count = p_shading_rate_palettes.len() as u32;
        (fp)(
            Some(command_buffer),
            first_viewport,
            viewport_count,
            p_shading_rate_palettes.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_order_type: vk::CoarseSampleOrderTypeNV,
        p_custom_sample_orders: &[vk::CoarseSampleOrderCustomNV],
    ) {
        let fp = self
            .fp_cmd_set_coarse_sample_order_nv
            .expect("vkCmdSetCoarseSampleOrderNV is not loaded");
        let custom_sample_order_count = p_custom_sample_orders.len() as u32;
        (fp)(
            Some(command_buffer),
            sample_order_type,
            custom_sample_order_count,
            p_custom_sample_orders.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_draw_mesh_tasks_nv(&self, command_buffer: vk::CommandBuffer, task_count: u32, first_task: u32) {
        let fp = self
            .fp_cmd_draw_mesh_tasks_nv
            .expect("vkCmdDrawMeshTasksNV is not loaded");
        (fp)(Some(command_buffer), task_count, first_task);
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_mesh_tasks_indirect_nv
            .expect("vkCmdDrawMeshTasksIndirectNV is not loaded");
        (fp)(Some(command_buffer), Some(buffer), offset, draw_count, stride);
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let fp = self
            .fp_cmd_draw_mesh_tasks_indirect_count_nv
            .expect("vkCmdDrawMeshTasksIndirectCountNV is not loaded");
        (fp)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn compile_deferred_nv(&self, pipeline: vk::Pipeline, shader: u32) -> Result<()> {
        let fp = self.fp_compile_deferred_nv.expect("vkCompileDeferredNV is not loaded");
        let err = (fp)(Some(self.handle), Some(pipeline), shader);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_acceleration_structure_nv(
        &self,
        p_create_info: &vk::AccelerationStructureCreateInfoNV,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::AccelerationStructureNV> {
        let fp = self
            .fp_create_acceleration_structure_nv
            .expect("vkCreateAccelerationStructureNV is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_acceleration_structure_khr(
        &self,
        acceleration_structure: Option<vk::AccelerationStructureKHR>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_acceleration_structure_khr
            .expect("vkDestroyAccelerationStructureKHR is not loaded");
        (fp)(
            Some(self.handle),
            acceleration_structure,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn destroy_acceleration_structure_nv(
        &self,
        acceleration_structure: Option<vk::AccelerationStructureNV>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_acceleration_structure_nv
            .expect("vkDestroyAccelerationStructureNV is not loaded");
        (fp)(
            Some(self.handle),
            acceleration_structure,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_acceleration_structure_memory_requirements_nv(
        &self,
        p_info: &vk::AccelerationStructureMemoryRequirementsInfoNV,
        p_memory_requirements: &mut vk::MemoryRequirements2KHR,
    ) {
        let fp = self
            .fp_get_acceleration_structure_memory_requirements_nv
            .expect("vkGetAccelerationStructureMemoryRequirementsNV is not loaded");
        (fp)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn bind_acceleration_structure_memory_nv(
        &self,
        p_bind_infos: &[vk::BindAccelerationStructureMemoryInfoNV],
    ) -> Result<()> {
        let fp = self
            .fp_bind_acceleration_structure_memory_nv
            .expect("vkBindAccelerationStructureMemoryNV is not loaded");
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (fp)(
            Some(self.handle),
            bind_info_count,
            p_bind_infos.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        mode: vk::CopyAccelerationStructureModeKHR,
    ) {
        let fp = self
            .fp_cmd_copy_acceleration_structure_nv
            .expect("vkCmdCopyAccelerationStructureNV is not loaded");
        (fp)(Some(command_buffer), Some(dst), Some(src), mode);
    }
    pub unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_info: &vk::CopyAccelerationStructureInfoKHR,
    ) {
        let fp = self
            .fp_cmd_copy_acceleration_structure_khr
            .expect("vkCmdCopyAccelerationStructureKHR is not loaded");
        (fp)(Some(command_buffer), p_info);
    }
    pub unsafe fn copy_acceleration_structure_khr(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        p_info: &vk::CopyAccelerationStructureInfoKHR,
    ) -> Result<vk::Result> {
        let fp = self
            .fp_copy_acceleration_structure_khr
            .expect("vkCopyAccelerationStructureKHR is not loaded");
        let err = (fp)(Some(self.handle), deferred_operation, p_info);
        match err {
            vk::Result::SUCCESS | vk::Result::OPERATION_DEFERRED_KHR | vk::Result::OPERATION_NOT_DEFERRED_KHR => {
                Ok(err)
            }
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_info: &vk::CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        let fp = self
            .fp_cmd_copy_acceleration_structure_to_memory_khr
            .expect("vkCmdCopyAccelerationStructureToMemoryKHR is not loaded");
        (fp)(Some(command_buffer), p_info);
    }
    pub unsafe fn copy_acceleration_structure_to_memory_khr(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        p_info: &vk::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> Result<vk::Result> {
        let fp = self
            .fp_copy_acceleration_structure_to_memory_khr
            .expect("vkCopyAccelerationStructureToMemoryKHR is not loaded");
        let err = (fp)(Some(self.handle), deferred_operation, p_info);
        match err {
            vk::Result::SUCCESS | vk::Result::OPERATION_DEFERRED_KHR | vk::Result::OPERATION_NOT_DEFERRED_KHR => {
                Ok(err)
            }
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_info: &vk::CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        let fp = self
            .fp_cmd_copy_memory_to_acceleration_structure_khr
            .expect("vkCmdCopyMemoryToAccelerationStructureKHR is not loaded");
        (fp)(Some(command_buffer), p_info);
    }
    pub unsafe fn copy_memory_to_acceleration_structure_khr(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        p_info: &vk::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> Result<vk::Result> {
        let fp = self
            .fp_copy_memory_to_acceleration_structure_khr
            .expect("vkCopyMemoryToAccelerationStructureKHR is not loaded");
        let err = (fp)(Some(self.handle), deferred_operation, p_info);
        match err {
            vk::Result::SUCCESS | vk::Result::OPERATION_DEFERRED_KHR | vk::Result::OPERATION_NOT_DEFERRED_KHR => {
                Ok(err)
            }
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_acceleration_structures: &[vk::AccelerationStructureKHR],
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        let fp = self
            .fp_cmd_write_acceleration_structures_properties_khr
            .expect("vkCmdWriteAccelerationStructuresPropertiesKHR is not loaded");
        let acceleration_structure_count = p_acceleration_structures.len() as u32;
        (fp)(
            Some(command_buffer),
            acceleration_structure_count,
            p_acceleration_structures.first().map_or(ptr::null(), |s| s as *const _),
            query_type,
            Some(query_pool),
            first_query,
        );
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        p_acceleration_structures: &[vk::AccelerationStructureNV],
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        let fp = self
            .fp_cmd_write_acceleration_structures_properties_nv
            .expect("vkCmdWriteAccelerationStructuresPropertiesNV is not loaded");
        let acceleration_structure_count = p_acceleration_structures.len() as u32;
        (fp)(
            Some(command_buffer),
            acceleration_structure_count,
            p_acceleration_structures.first().map_or(ptr::null(), |s| s as *const _),
            query_type,
            Some(query_pool),
            first_query,
        );
    }
    pub unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        p_info: &vk::AccelerationStructureInfoNV,
        instance_data: Option<vk::Buffer>,
        instance_offset: vk::DeviceSize,
        update: bool,
        dst: vk::AccelerationStructureNV,
        src: Option<vk::AccelerationStructureNV>,
        scratch: vk::Buffer,
        scratch_offset: vk::DeviceSize,
    ) {
        let fp = self
            .fp_cmd_build_acceleration_structure_nv
            .expect("vkCmdBuildAccelerationStructureNV is not loaded");
        (fp)(
            Some(command_buffer),
            p_info,
            instance_data,
            instance_offset,
            if update { vk::TRUE } else { vk::FALSE },
            Some(dst),
            src,
            Some(scratch),
            scratch_offset,
        );
    }
    pub unsafe fn write_acceleration_structures_properties_khr<T>(
        &self,
        p_acceleration_structures: &[vk::AccelerationStructureKHR],
        query_type: vk::QueryType,
        p_data: &mut [T],
        stride: usize,
    ) -> Result<()> {
        let fp = self
            .fp_write_acceleration_structures_properties_khr
            .expect("vkWriteAccelerationStructuresPropertiesKHR is not loaded");
        let acceleration_structure_count = p_acceleration_structures.len() as u32;
        let data_size = mem::size_of_val(p_data) as usize;
        let err = (fp)(
            Some(self.handle),
            acceleration_structure_count,
            p_acceleration_structures.first().map_or(ptr::null(), |s| s as *const _),
            query_type,
            data_size,
            p_data.first_mut().map_or(ptr::null_mut(), |s| s as *mut _) as *mut _,
            stride,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_raygen_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        let fp = self.fp_cmd_trace_rays_khr.expect("vkCmdTraceRaysKHR is not loaded");
        (fp)(
            Some(command_buffer),
            p_raygen_shader_binding_table,
            p_miss_shader_binding_table,
            p_hit_shader_binding_table,
            p_callable_shader_binding_table,
            width,
            height,
            depth,
        );
    }
    pub unsafe fn cmd_trace_rays_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table_buffer: vk::Buffer,
        raygen_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_table_buffer: Option<vk::Buffer>,
        miss_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_stride: vk::DeviceSize,
        hit_shader_binding_table_buffer: Option<vk::Buffer>,
        hit_shader_binding_offset: vk::DeviceSize,
        hit_shader_binding_stride: vk::DeviceSize,
        callable_shader_binding_table_buffer: Option<vk::Buffer>,
        callable_shader_binding_offset: vk::DeviceSize,
        callable_shader_binding_stride: vk::DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        let fp = self.fp_cmd_trace_rays_nv.expect("vkCmdTraceRaysNV is not loaded");
        (fp)(
            Some(command_buffer),
            Some(raygen_shader_binding_table_buffer),
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        );
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_khr<T>(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        p_data: &mut [T],
    ) -> Result<()> {
        let fp = self
            .fp_get_ray_tracing_shader_group_handles_khr
            .expect("vkGetRayTracingShaderGroupHandlesKHR is not loaded");
        let data_size = mem::size_of_val(p_data) as usize;
        let err = (fp)(
            Some(self.handle),
            Some(pipeline),
            first_group,
            group_count,
            data_size,
            p_data.first_mut().map_or(ptr::null_mut(), |s| s as *mut _) as *mut _,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_nv<T>(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        p_data: &mut [T],
    ) -> Result<()> {
        let fp = self
            .fp_get_ray_tracing_shader_group_handles_khr
            .expect("vkGetRayTracingShaderGroupHandlesNV is not loaded");
        let data_size = mem::size_of_val(p_data) as usize;
        let err = (fp)(
            Some(self.handle),
            Some(pipeline),
            first_group,
            group_count,
            data_size,
            p_data.first_mut().map_or(ptr::null_mut(), |s| s as *mut _) as *mut _,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr<T>(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        p_data: &mut [T],
    ) -> Result<()> {
        let fp = self
            .fp_get_ray_tracing_capture_replay_shader_group_handles_khr
            .expect("vkGetRayTracingCaptureReplayShaderGroupHandlesKHR is not loaded");
        let data_size = mem::size_of_val(p_data) as usize;
        let err = (fp)(
            Some(self.handle),
            Some(pipeline),
            first_group,
            group_count,
            data_size,
            p_data.first_mut().map_or(ptr::null_mut(), |s| s as *mut _) as *mut _,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_acceleration_structure_handle_nv<T>(
        &self,
        acceleration_structure: vk::AccelerationStructureNV,
        p_data: &mut [T],
    ) -> Result<()> {
        let fp = self
            .fp_get_acceleration_structure_handle_nv
            .expect("vkGetAccelerationStructureHandleNV is not loaded");
        let data_size = mem::size_of_val(p_data) as usize;
        let err = (fp)(
            Some(self.handle),
            Some(acceleration_structure),
            data_size,
            p_data.first_mut().map_or(ptr::null_mut(), |s| s as *mut _) as *mut _,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoNV],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<()> {
        let fp = self
            .fp_create_ray_tracing_pipelines_nv
            .expect("vkCreateRayTracingPipelinesNV is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            p_pipelines,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_nv_to_vec(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoNV],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        let fp = self
            .fp_create_ray_tracing_pipelines_nv
            .expect("vkCreateRayTracingPipelinesNV is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(create_info_count as usize);
        v.set_len(create_info_count as usize);
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_nv_array<const N: usize>(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoNV],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<[vk::Pipeline; N]> {
        let fp = self
            .fp_create_ray_tracing_pipelines_nv
            .expect("vkCreateRayTracingPipelinesNV is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, N as u32);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr() as *mut _,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_nv_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &vk::RayTracingPipelineCreateInfoNV,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        let fp = self
            .fp_create_ray_tracing_pipelines_nv
            .expect("vkCreateRayTracingPipelinesNV is not loaded");
        let create_info_count = 1;
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<()> {
        let fp = self
            .fp_create_ray_tracing_pipelines_khr
            .expect("vkCreateRayTracingPipelinesKHR is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        let v_err = (fp)(
            Some(self.handle),
            deferred_operation,
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            p_pipelines,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_khr_to_vec(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        let fp = self
            .fp_create_ray_tracing_pipelines_khr
            .expect("vkCreateRayTracingPipelinesKHR is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(create_info_count as usize);
        v.set_len(create_info_count as usize);
        let v_err = (fp)(
            Some(self.handle),
            deferred_operation,
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_khr_array<const N: usize>(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<[vk::Pipeline; N]> {
        let fp = self
            .fp_create_ray_tracing_pipelines_khr
            .expect("vkCreateRayTracingPipelinesKHR is not loaded");
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, N as u32);
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            deferred_operation,
            pipeline_cache,
            create_info_count,
            p_create_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr() as *mut _,
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_khr_single(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &vk::RayTracingPipelineCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        let fp = self
            .fp_create_ray_tracing_pipelines_khr
            .expect("vkCreateRayTracingPipelinesKHR is not loaded");
        let create_info_count = 1;
        let mut v = MaybeUninit::<_>::uninit();
        let v_err = (fp)(
            Some(self.handle),
            deferred_operation,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        match v_err {
            vk::Result::SUCCESS => Ok(v.assume_init()),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::CooperativeMatrixPropertiesNV>> {
        let fp = self
            .fp_get_physical_device_cooperative_matrix_properties_nv
            .expect("vkGetPhysicalDeviceCooperativeMatrixPropertiesNV is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_raygen_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        indirect_device_address: vk::DeviceAddress,
    ) {
        let fp = self
            .fp_cmd_trace_rays_indirect_khr
            .expect("vkCmdTraceRaysIndirectKHR is not loaded");
        (fp)(
            Some(command_buffer),
            p_raygen_shader_binding_table,
            p_miss_shader_binding_table,
            p_hit_shader_binding_table,
            p_callable_shader_binding_table,
            indirect_device_address,
        );
    }
    pub unsafe fn get_device_acceleration_structure_compatibility_khr(
        &self,
        p_version_info: &vk::AccelerationStructureVersionInfoKHR,
    ) -> vk::AccelerationStructureCompatibilityKHR {
        let fp = self
            .fp_get_device_acceleration_structure_compatibility_khr
            .expect("vkGetDeviceAccelerationStructureCompatibilityKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(self.handle), p_version_info, res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
        &self,
        pipeline: vk::Pipeline,
        group: u32,
        group_shader: vk::ShaderGroupShaderKHR,
    ) -> vk::DeviceSize {
        let fp = self
            .fp_get_ray_tracing_shader_group_stack_size_khr
            .expect("vkGetRayTracingShaderGroupStackSizeKHR is not loaded");
        (fp)(Some(self.handle), Some(pipeline), group, group_shader)
    }
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        let fp = self
            .fp_cmd_set_ray_tracing_pipeline_stack_size_khr
            .expect("vkCmdSetRayTracingPipelineStackSizeKHR is not loaded");
        (fp)(Some(command_buffer), pipeline_stack_size);
    }
    pub unsafe fn get_image_view_handle_nvx(&self, p_info: &vk::ImageViewHandleInfoNVX) -> u32 {
        let fp = self
            .fp_get_image_view_handle_nvx
            .expect("vkGetImageViewHandleNVX is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn get_image_view_address_nvx(
        &self,
        image_view: vk::ImageView,
        p_properties: &mut vk::ImageViewAddressPropertiesNVX,
    ) -> Result<()> {
        let fp = self
            .fp_get_image_view_address_nvx
            .expect("vkGetImageViewAddressNVX is not loaded");
        let err = (fp)(Some(self.handle), Some(image_view), p_properties);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_surface_present_modes2_ext_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<Vec<vk::PresentModeKHR>> {
        let fp = self
            .fp_get_physical_device_surface_present_modes2_ext
            .expect("vkGetPhysicalDeviceSurfacePresentModes2EXT is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), p_surface_info, len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), p_surface_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<vk::DeviceGroupPresentModeFlagsKHR> {
        let fp = self
            .fp_get_device_group_surface_present_modes2_ext
            .expect("vkGetDeviceGroupSurfacePresentModes2EXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_surface_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(&self, swapchain: vk::SwapchainKHR) -> Result<()> {
        let fp = self
            .fp_acquire_full_screen_exclusive_mode_ext
            .expect("vkAcquireFullScreenExclusiveModeEXT is not loaded");
        let err = (fp)(Some(self.handle), Some(swapchain));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn release_full_screen_exclusive_mode_ext(&self, swapchain: vk::SwapchainKHR) -> Result<()> {
        let fp = self
            .fp_release_full_screen_exclusive_mode_ext
            .expect("vkReleaseFullScreenExclusiveModeEXT is not loaded");
        let err = (fp)(Some(self.handle), Some(swapchain));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: &mut u32,
        p_counters: *mut vk::PerformanceCounterKHR,
        p_counter_descriptions: *mut vk::PerformanceCounterDescriptionKHR,
    ) -> Result<vk::Result> {
        let fp = self
            .fp_enumerate_physical_device_queue_family_performance_query_counters_khr
            .expect("vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR is not loaded");
        let err = (fp)(
            Some(physical_device),
            queue_family_index,
            p_counter_count,
            p_counters,
            p_counter_descriptions,
        );
        match err {
            vk::Result::SUCCESS | vk::Result::INCOMPLETE => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_performance_query_create_info: &vk::QueryPoolPerformanceCreateInfoKHR,
    ) -> u32 {
        let fp = self
            .fp_get_physical_device_queue_family_performance_query_passes_khr
            .expect("vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(Some(physical_device), p_performance_query_create_info, res.as_mut_ptr());
        res.assume_init()
    }
    pub unsafe fn acquire_profiling_lock_khr(&self, p_info: &vk::AcquireProfilingLockInfoKHR) -> Result<()> {
        let fp = self
            .fp_acquire_profiling_lock_khr
            .expect("vkAcquireProfilingLockKHR is not loaded");
        let err = (fp)(Some(self.handle), p_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn release_profiling_lock_khr(&self) {
        let fp = self
            .fp_release_profiling_lock_khr
            .expect("vkReleaseProfilingLockKHR is not loaded");
        (fp)(Some(self.handle));
    }
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        image: vk::Image,
        p_properties: &mut vk::ImageDrmFormatModifierPropertiesEXT,
    ) -> Result<()> {
        let fp = self
            .fp_get_image_drm_format_modifier_properties_ext
            .expect("vkGetImageDrmFormatModifierPropertiesEXT is not loaded");
        let err = (fp)(Some(self.handle), Some(image), p_properties);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_buffer_opaque_capture_address(&self, p_info: &vk::BufferDeviceAddressInfo) -> u64 {
        let fp = self
            .fp_get_buffer_opaque_capture_address
            .expect("vkGetBufferOpaqueCaptureAddress is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn get_buffer_opaque_capture_address_khr(&self, p_info: &vk::BufferDeviceAddressInfo) -> u64 {
        let fp = self
            .fp_get_buffer_opaque_capture_address
            .expect("vkGetBufferOpaqueCaptureAddressKHR is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn get_buffer_device_address(&self, p_info: &vk::BufferDeviceAddressInfo) -> vk::DeviceAddress {
        let fp = self
            .fp_get_buffer_device_address
            .expect("vkGetBufferDeviceAddress is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn get_buffer_device_address_khr(&self, p_info: &vk::BufferDeviceAddressInfo) -> vk::DeviceAddress {
        let fp = self
            .fp_get_buffer_device_address
            .expect("vkGetBufferDeviceAddressKHR is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn get_buffer_device_address_ext(&self, p_info: &vk::BufferDeviceAddressInfo) -> vk::DeviceAddress {
        let fp = self
            .fp_get_buffer_device_address
            .expect("vkGetBufferDeviceAddressEXT is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::FramebufferMixedSamplesCombinationNV>> {
        let fp = self
            .fp_get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
            .expect("vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn initialize_performance_api_intel(
        &self,
        p_initialize_info: &vk::InitializePerformanceApiInfoINTEL,
    ) -> Result<()> {
        let fp = self
            .fp_initialize_performance_api_intel
            .expect("vkInitializePerformanceApiINTEL is not loaded");
        let err = (fp)(Some(self.handle), p_initialize_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn uninitialize_performance_api_intel(&self) {
        let fp = self
            .fp_uninitialize_performance_api_intel
            .expect("vkUninitializePerformanceApiINTEL is not loaded");
        (fp)(Some(self.handle));
    }
    pub unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: vk::CommandBuffer,
        p_marker_info: &vk::PerformanceMarkerInfoINTEL,
    ) -> Result<()> {
        let fp = self
            .fp_cmd_set_performance_marker_intel
            .expect("vkCmdSetPerformanceMarkerINTEL is not loaded");
        let err = (fp)(Some(command_buffer), p_marker_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: vk::CommandBuffer,
        p_marker_info: &vk::PerformanceStreamMarkerInfoINTEL,
    ) -> Result<()> {
        let fp = self
            .fp_cmd_set_performance_stream_marker_intel
            .expect("vkCmdSetPerformanceStreamMarkerINTEL is not loaded");
        let err = (fp)(Some(command_buffer), p_marker_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: vk::CommandBuffer,
        p_override_info: &vk::PerformanceOverrideInfoINTEL,
    ) -> Result<()> {
        let fp = self
            .fp_cmd_set_performance_override_intel
            .expect("vkCmdSetPerformanceOverrideINTEL is not loaded");
        let err = (fp)(Some(command_buffer), p_override_info);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn acquire_performance_configuration_intel(
        &self,
        p_acquire_info: &vk::PerformanceConfigurationAcquireInfoINTEL,
    ) -> Result<vk::PerformanceConfigurationINTEL> {
        let fp = self
            .fp_acquire_performance_configuration_intel
            .expect("vkAcquirePerformanceConfigurationINTEL is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), p_acquire_info, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn release_performance_configuration_intel(
        &self,
        configuration: Option<vk::PerformanceConfigurationINTEL>,
    ) -> Result<()> {
        let fp = self
            .fp_release_performance_configuration_intel
            .expect("vkReleasePerformanceConfigurationINTEL is not loaded");
        let err = (fp)(Some(self.handle), configuration);
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: vk::Queue,
        configuration: vk::PerformanceConfigurationINTEL,
    ) -> Result<()> {
        let fp = self
            .fp_queue_set_performance_configuration_intel
            .expect("vkQueueSetPerformanceConfigurationINTEL is not loaded");
        let err = (fp)(Some(queue), Some(configuration));
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_performance_parameter_intel(
        &self,
        parameter: vk::PerformanceParameterTypeINTEL,
    ) -> Result<vk::PerformanceValueINTEL> {
        let fp = self
            .fp_get_performance_parameter_intel
            .expect("vkGetPerformanceParameterINTEL is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(Some(self.handle), parameter, res.as_mut_ptr());
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        p_info: &vk::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        let fp = self
            .fp_get_device_memory_opaque_capture_address
            .expect("vkGetDeviceMemoryOpaqueCaptureAddress is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn get_device_memory_opaque_capture_address_khr(
        &self,
        p_info: &vk::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        let fp = self
            .fp_get_device_memory_opaque_capture_address
            .expect("vkGetDeviceMemoryOpaqueCaptureAddressKHR is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn get_pipeline_executable_properties_khr_to_vec(
        &self,
        p_pipeline_info: &vk::PipelineInfoKHR,
    ) -> Result<Vec<vk::PipelineExecutablePropertiesKHR>> {
        let fp = self
            .fp_get_pipeline_executable_properties_khr
            .expect("vkGetPipelineExecutablePropertiesKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(self.handle), p_pipeline_info, len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(self.handle), p_pipeline_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_pipeline_executable_statistics_khr_to_vec(
        &self,
        p_executable_info: &vk::PipelineExecutableInfoKHR,
    ) -> Result<Vec<vk::PipelineExecutableStatisticKHR>> {
        let fp = self
            .fp_get_pipeline_executable_statistics_khr
            .expect("vkGetPipelineExecutableStatisticsKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(self.handle), p_executable_info, len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(self.handle), p_executable_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn get_pipeline_executable_internal_representations_khr_to_vec(
        &self,
        p_executable_info: &vk::PipelineExecutableInfoKHR,
    ) -> Result<Vec<vk::PipelineExecutableInternalRepresentationKHR>> {
        let fp = self
            .fp_get_pipeline_executable_internal_representations_khr
            .expect("vkGetPipelineExecutableInternalRepresentationsKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(self.handle), p_executable_info, len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(self.handle), p_executable_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        let fp = self
            .fp_cmd_set_line_stipple_ext
            .expect("vkCmdSetLineStippleEXT is not loaded");
        (fp)(Some(command_buffer), line_stipple_factor, line_stipple_pattern);
    }
    pub unsafe fn get_physical_device_tool_properties_ext_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::PhysicalDeviceToolPropertiesEXT>> {
        let fp = self
            .fp_get_physical_device_tool_properties_ext
            .expect("vkGetPhysicalDeviceToolPropertiesEXT is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn create_acceleration_structure_khr(
        &self,
        p_create_info: &vk::AccelerationStructureCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::AccelerationStructureKHR> {
        let fp = self
            .fp_create_acceleration_structure_khr
            .expect("vkCreateAccelerationStructureKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_build_acceleration_structures_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_infos: &[vk::AccelerationStructureBuildGeometryInfoKHR],
        pp_build_range_infos: &[*const vk::AccelerationStructureBuildRangeInfoKHR],
    ) {
        let fp = self
            .fp_cmd_build_acceleration_structures_khr
            .expect("vkCmdBuildAccelerationStructuresKHR is not loaded");
        let info_count = p_infos.len() as u32;
        assert_eq!(info_count, pp_build_range_infos.len() as u32);
        (fp)(
            Some(command_buffer),
            info_count,
            p_infos.first().map_or(ptr::null(), |s| s as *const _),
            pp_build_range_infos.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_infos: &[vk::AccelerationStructureBuildGeometryInfoKHR],
        p_indirect_device_addresses: &[vk::DeviceAddress],
        p_indirect_strides: &[u32],
        pp_max_primitive_counts: &[*const u32],
    ) {
        let fp = self
            .fp_cmd_build_acceleration_structures_indirect_khr
            .expect("vkCmdBuildAccelerationStructuresIndirectKHR is not loaded");
        let info_count = p_infos.len() as u32;
        assert_eq!(info_count, p_indirect_device_addresses.len() as u32);
        assert_eq!(info_count, p_indirect_strides.len() as u32);
        assert_eq!(info_count, pp_max_primitive_counts.len() as u32);
        (fp)(
            Some(command_buffer),
            info_count,
            p_infos.first().map_or(ptr::null(), |s| s as *const _),
            p_indirect_device_addresses
                .first()
                .map_or(ptr::null(), |s| s as *const _),
            p_indirect_strides.first().map_or(ptr::null(), |s| s as *const _),
            pp_max_primitive_counts.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn build_acceleration_structures_khr(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        p_infos: &[vk::AccelerationStructureBuildGeometryInfoKHR],
        pp_build_range_infos: &[*const vk::AccelerationStructureBuildRangeInfoKHR],
    ) -> Result<vk::Result> {
        let fp = self
            .fp_build_acceleration_structures_khr
            .expect("vkBuildAccelerationStructuresKHR is not loaded");
        let info_count = p_infos.len() as u32;
        assert_eq!(info_count, pp_build_range_infos.len() as u32);
        let err = (fp)(
            Some(self.handle),
            deferred_operation,
            info_count,
            p_infos.first().map_or(ptr::null(), |s| s as *const _),
            pp_build_range_infos.first().map_or(ptr::null(), |s| s as *const _),
        );
        match err {
            vk::Result::SUCCESS | vk::Result::OPERATION_DEFERRED_KHR | vk::Result::OPERATION_NOT_DEFERRED_KHR => {
                Ok(err)
            }
            _ => Err(err),
        }
    }
    pub unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        p_info: &vk::AccelerationStructureDeviceAddressInfoKHR,
    ) -> vk::DeviceAddress {
        let fp = self
            .fp_get_acceleration_structure_device_address_khr
            .expect("vkGetAccelerationStructureDeviceAddressKHR is not loaded");
        (fp)(Some(self.handle), p_info)
    }
    pub unsafe fn create_deferred_operation_khr(
        &self,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DeferredOperationKHR> {
        let fp = self
            .fp_create_deferred_operation_khr
            .expect("vkCreateDeferredOperationKHR is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_deferred_operation_khr(
        &self,
        operation: Option<vk::DeferredOperationKHR>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_deferred_operation_khr
            .expect("vkDestroyDeferredOperationKHR is not loaded");
        (fp)(Some(self.handle), operation, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_deferred_operation_max_concurrency_khr(&self, operation: vk::DeferredOperationKHR) -> u32 {
        let fp = self
            .fp_get_deferred_operation_max_concurrency_khr
            .expect("vkGetDeferredOperationMaxConcurrencyKHR is not loaded");
        (fp)(Some(self.handle), Some(operation))
    }
    pub unsafe fn get_deferred_operation_result_khr(&self, operation: vk::DeferredOperationKHR) -> Result<vk::Result> {
        let fp = self
            .fp_get_deferred_operation_result_khr
            .expect("vkGetDeferredOperationResultKHR is not loaded");
        let err = (fp)(Some(self.handle), Some(operation));
        match err {
            vk::Result::SUCCESS | vk::Result::NOT_READY => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn deferred_operation_join_khr(&self, operation: vk::DeferredOperationKHR) -> Result<vk::Result> {
        let fp = self
            .fp_deferred_operation_join_khr
            .expect("vkDeferredOperationJoinKHR is not loaded");
        let err = (fp)(Some(self.handle), Some(operation));
        match err {
            vk::Result::SUCCESS | vk::Result::THREAD_DONE_KHR | vk::Result::THREAD_IDLE_KHR => Ok(err),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_set_cull_mode_ext(&self, command_buffer: vk::CommandBuffer, cull_mode: vk::CullModeFlags) {
        let fp = self
            .fp_cmd_set_cull_mode_ext
            .expect("vkCmdSetCullModeEXT is not loaded");
        (fp)(Some(command_buffer), cull_mode);
    }
    pub unsafe fn cmd_set_front_face_ext(&self, command_buffer: vk::CommandBuffer, front_face: vk::FrontFace) {
        let fp = self
            .fp_cmd_set_front_face_ext
            .expect("vkCmdSetFrontFaceEXT is not loaded");
        (fp)(Some(command_buffer), front_face);
    }
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_topology: vk::PrimitiveTopology,
    ) {
        let fp = self
            .fp_cmd_set_primitive_topology_ext
            .expect("vkCmdSetPrimitiveTopologyEXT is not loaded");
        (fp)(Some(command_buffer), primitive_topology);
    }
    pub unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_viewports: &[vk::Viewport],
    ) {
        let fp = self
            .fp_cmd_set_viewport_with_count_ext
            .expect("vkCmdSetViewportWithCountEXT is not loaded");
        let viewport_count = p_viewports.len() as u32;
        (fp)(
            Some(command_buffer),
            viewport_count,
            p_viewports.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_scissor_with_count_ext(&self, command_buffer: vk::CommandBuffer, p_scissors: &[vk::Rect2D]) {
        let fp = self
            .fp_cmd_set_scissor_with_count_ext
            .expect("vkCmdSetScissorWithCountEXT is not loaded");
        let scissor_count = p_scissors.len() as u32;
        (fp)(
            Some(command_buffer),
            scissor_count,
            p_scissors.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_bind_vertex_buffers2_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        p_buffers: &[vk::Buffer],
        p_offsets: &[vk::DeviceSize],
        p_sizes: Option<&[vk::DeviceSize]>,
        p_strides: Option<&[vk::DeviceSize]>,
    ) {
        let fp = self
            .fp_cmd_bind_vertex_buffers2_ext
            .expect("vkCmdBindVertexBuffers2EXT is not loaded");
        let binding_count = p_buffers.len() as u32;
        assert_eq!(binding_count, p_offsets.len() as u32);
        if let Some(s) = p_sizes {
            assert_eq!(binding_count, s.len() as u32);
        }
        if let Some(s) = p_strides {
            assert_eq!(binding_count, s.len() as u32);
        }
        (fp)(
            Some(command_buffer),
            first_binding,
            binding_count,
            p_buffers.first().map_or(ptr::null(), |s| s as *const _),
            p_offsets.first().map_or(ptr::null(), |s| s as *const _),
            p_sizes.and_then(|s| s.first()).map_or(ptr::null(), |s| s as *const _),
            p_strides.and_then(|s| s.first()).map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_depth_test_enable_ext(&self, command_buffer: vk::CommandBuffer, depth_test_enable: bool) {
        let fp = self
            .fp_cmd_set_depth_test_enable_ext
            .expect("vkCmdSetDepthTestEnableEXT is not loaded");
        (fp)(
            Some(command_buffer),
            if depth_test_enable { vk::TRUE } else { vk::FALSE },
        );
    }
    pub unsafe fn cmd_set_depth_write_enable_ext(&self, command_buffer: vk::CommandBuffer, depth_write_enable: bool) {
        let fp = self
            .fp_cmd_set_depth_write_enable_ext
            .expect("vkCmdSetDepthWriteEnableEXT is not loaded");
        (fp)(
            Some(command_buffer),
            if depth_write_enable { vk::TRUE } else { vk::FALSE },
        );
    }
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_compare_op: vk::CompareOp,
    ) {
        let fp = self
            .fp_cmd_set_depth_compare_op_ext
            .expect("vkCmdSetDepthCompareOpEXT is not loaded");
        (fp)(Some(command_buffer), depth_compare_op);
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        let fp = self
            .fp_cmd_set_depth_bounds_test_enable_ext
            .expect("vkCmdSetDepthBoundsTestEnableEXT is not loaded");
        (fp)(
            Some(command_buffer),
            if depth_bounds_test_enable { vk::TRUE } else { vk::FALSE },
        );
    }
    pub unsafe fn cmd_set_stencil_test_enable_ext(&self, command_buffer: vk::CommandBuffer, stencil_test_enable: bool) {
        let fp = self
            .fp_cmd_set_stencil_test_enable_ext
            .expect("vkCmdSetStencilTestEnableEXT is not loaded");
        (fp)(
            Some(command_buffer),
            if stencil_test_enable { vk::TRUE } else { vk::FALSE },
        );
    }
    pub unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        fail_op: vk::StencilOp,
        pass_op: vk::StencilOp,
        depth_fail_op: vk::StencilOp,
        compare_op: vk::CompareOp,
    ) {
        let fp = self
            .fp_cmd_set_stencil_op_ext
            .expect("vkCmdSetStencilOpEXT is not loaded");
        (fp)(
            Some(command_buffer),
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        );
    }
    pub unsafe fn create_private_data_slot_ext(
        &self,
        p_create_info: &vk::PrivateDataSlotCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::PrivateDataSlotEXT> {
        let fp = self
            .fp_create_private_data_slot_ext
            .expect("vkCreatePrivateDataSlotEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        let err = (fp)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            res.as_mut_ptr(),
        );
        match err {
            vk::Result::SUCCESS => Ok(res.assume_init()),
            _ => Err(err),
        }
    }
    pub unsafe fn destroy_private_data_slot_ext(
        &self,
        private_data_slot: Option<vk::PrivateDataSlotEXT>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        let fp = self
            .fp_destroy_private_data_slot_ext
            .expect("vkDestroyPrivateDataSlotEXT is not loaded");
        (fp)(
            Some(self.handle),
            private_data_slot,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn set_private_data_ext(
        &self,
        object_type: vk::ObjectType,
        object_handle: u64,
        private_data_slot: vk::PrivateDataSlotEXT,
        data: u64,
    ) -> Result<()> {
        let fp = self.fp_set_private_data_ext.expect("vkSetPrivateDataEXT is not loaded");
        let err = (fp)(
            Some(self.handle),
            object_type,
            object_handle,
            Some(private_data_slot),
            data,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn get_private_data_ext(
        &self,
        object_type: vk::ObjectType,
        object_handle: u64,
        private_data_slot: vk::PrivateDataSlotEXT,
    ) -> u64 {
        let fp = self.fp_get_private_data_ext.expect("vkGetPrivateDataEXT is not loaded");
        let mut res = MaybeUninit::<_>::uninit();
        (fp)(
            Some(self.handle),
            object_type,
            object_handle,
            Some(private_data_slot),
            res.as_mut_ptr(),
        );
        res.assume_init()
    }
    pub unsafe fn cmd_copy_buffer2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_copy_buffer_info: &vk::CopyBufferInfo2KHR,
    ) {
        let fp = self.fp_cmd_copy_buffer2_khr.expect("vkCmdCopyBuffer2KHR is not loaded");
        (fp)(Some(command_buffer), p_copy_buffer_info);
    }
    pub unsafe fn cmd_copy_image2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_copy_image_info: &vk::CopyImageInfo2KHR,
    ) {
        let fp = self.fp_cmd_copy_image2_khr.expect("vkCmdCopyImage2KHR is not loaded");
        (fp)(Some(command_buffer), p_copy_image_info);
    }
    pub unsafe fn cmd_blit_image2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_blit_image_info: &vk::BlitImageInfo2KHR,
    ) {
        let fp = self.fp_cmd_blit_image2_khr.expect("vkCmdBlitImage2KHR is not loaded");
        (fp)(Some(command_buffer), p_blit_image_info);
    }
    pub unsafe fn cmd_copy_buffer_to_image2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_copy_buffer_to_image_info: &vk::CopyBufferToImageInfo2KHR,
    ) {
        let fp = self
            .fp_cmd_copy_buffer_to_image2_khr
            .expect("vkCmdCopyBufferToImage2KHR is not loaded");
        (fp)(Some(command_buffer), p_copy_buffer_to_image_info);
    }
    pub unsafe fn cmd_copy_image_to_buffer2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_copy_image_to_buffer_info: &vk::CopyImageToBufferInfo2KHR,
    ) {
        let fp = self
            .fp_cmd_copy_image_to_buffer2_khr
            .expect("vkCmdCopyImageToBuffer2KHR is not loaded");
        (fp)(Some(command_buffer), p_copy_image_to_buffer_info);
    }
    pub unsafe fn cmd_resolve_image2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_resolve_image_info: &vk::ResolveImageInfo2KHR,
    ) {
        let fp = self
            .fp_cmd_resolve_image2_khr
            .expect("vkCmdResolveImage2KHR is not loaded");
        (fp)(Some(command_buffer), p_resolve_image_info);
    }
    pub unsafe fn cmd_set_fragment_shading_rate_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_fragment_size: &vk::Extent2D,
        combiner_ops: [vk::FragmentShadingRateCombinerOpKHR; 2],
    ) {
        let fp = self
            .fp_cmd_set_fragment_shading_rate_khr
            .expect("vkCmdSetFragmentShadingRateKHR is not loaded");
        (fp)(Some(command_buffer), p_fragment_size, combiner_ops.as_ptr());
    }
    pub unsafe fn get_physical_device_fragment_shading_rates_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::PhysicalDeviceFragmentShadingRateKHR>> {
        let fp = self
            .fp_get_physical_device_fragment_shading_rates_khr
            .expect("vkGetPhysicalDeviceFragmentShadingRatesKHR is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        let len_err = (fp)(Some(physical_device), len.as_mut_ptr(), ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (fp)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        }
    }
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        shading_rate: vk::FragmentShadingRateNV,
        combiner_ops: [vk::FragmentShadingRateCombinerOpKHR; 2],
    ) {
        let fp = self
            .fp_cmd_set_fragment_shading_rate_enum_nv
            .expect("vkCmdSetFragmentShadingRateEnumNV is not loaded");
        (fp)(Some(command_buffer), shading_rate, combiner_ops.as_ptr());
    }
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        &self,
        build_type: vk::AccelerationStructureBuildTypeKHR,
        p_build_info: &vk::AccelerationStructureBuildGeometryInfoKHR,
        p_max_primitive_counts: Option<&[u32]>,
        p_size_info: &mut vk::AccelerationStructureBuildSizesInfoKHR,
    ) {
        let fp = self
            .fp_get_acceleration_structure_build_sizes_khr
            .expect("vkGetAccelerationStructureBuildSizesKHR is not loaded");
        if let Some(s) = p_max_primitive_counts {
            assert_eq!(s.len() as u32, p_build_info.geometry_count);
        }
        (fp)(
            Some(self.handle),
            build_type,
            p_build_info,
            p_max_primitive_counts
                .and_then(|s| s.first())
                .map_or(ptr::null(), |s| s as *const _),
            p_size_info,
        );
    }
    pub unsafe fn cmd_set_vertex_input_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_vertex_binding_descriptions: &[vk::VertexInputBindingDescription2EXT],
        p_vertex_attribute_descriptions: &[vk::VertexInputAttributeDescription2EXT],
    ) {
        let fp = self
            .fp_cmd_set_vertex_input_ext
            .expect("vkCmdSetVertexInputEXT is not loaded");
        let vertex_binding_description_count = p_vertex_binding_descriptions.len() as u32;
        let vertex_attribute_description_count = p_vertex_attribute_descriptions.len() as u32;
        (fp)(
            Some(command_buffer),
            vertex_binding_description_count,
            p_vertex_binding_descriptions
                .first()
                .map_or(ptr::null(), |s| s as *const _),
            vertex_attribute_description_count,
            p_vertex_attribute_descriptions
                .first()
                .map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_color_write_enable_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_color_write_enables: &[vk::Bool32],
    ) {
        let fp = self
            .fp_cmd_set_color_write_enable_ext
            .expect("vkCmdSetColorWriteEnableEXT is not loaded");
        let attachment_count = p_color_write_enables.len() as u32;
        (fp)(
            Some(command_buffer),
            attachment_count,
            p_color_write_enables.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_set_event2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        p_dependency_info: &vk::DependencyInfoKHR,
    ) {
        let fp = self.fp_cmd_set_event2_khr.expect("vkCmdSetEvent2KHR is not loaded");
        (fp)(Some(command_buffer), Some(event), p_dependency_info);
    }
    pub unsafe fn cmd_reset_event2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags2KHR,
    ) {
        let fp = self.fp_cmd_reset_event2_khr.expect("vkCmdResetEvent2KHR is not loaded");
        (fp)(Some(command_buffer), Some(event), stage_mask);
    }
    pub unsafe fn cmd_wait_events2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_events: &[vk::Event],
        p_dependency_infos: &[vk::DependencyInfoKHR],
    ) {
        let fp = self.fp_cmd_wait_events2_khr.expect("vkCmdWaitEvents2KHR is not loaded");
        let event_count = p_events.len() as u32;
        assert_eq!(event_count, p_dependency_infos.len() as u32);
        (fp)(
            Some(command_buffer),
            event_count,
            p_events.first().map_or(ptr::null(), |s| s as *const _),
            p_dependency_infos.first().map_or(ptr::null(), |s| s as *const _),
        );
    }
    pub unsafe fn cmd_pipeline_barrier2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_dependency_info: &vk::DependencyInfoKHR,
    ) {
        let fp = self
            .fp_cmd_pipeline_barrier2_khr
            .expect("vkCmdPipelineBarrier2KHR is not loaded");
        (fp)(Some(command_buffer), p_dependency_info);
    }
    pub unsafe fn queue_submit2_khr(
        &self,
        queue: vk::Queue,
        p_submits: &[vk::SubmitInfo2KHR],
        fence: Option<vk::Fence>,
    ) -> Result<()> {
        let fp = self.fp_queue_submit2_khr.expect("vkQueueSubmit2KHR is not loaded");
        let submit_count = p_submits.len() as u32;
        let err = (fp)(
            Some(queue),
            submit_count,
            p_submits.first().map_or(ptr::null(), |s| s as *const _),
            fence,
        );
        match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        }
    }
    pub unsafe fn cmd_write_timestamp2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        stage: vk::PipelineStageFlags2KHR,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        let fp = self
            .fp_cmd_write_timestamp2_khr
            .expect("vkCmdWriteTimestamp2KHR is not loaded");
        (fp)(Some(command_buffer), stage, Some(query_pool), query);
    }
    pub unsafe fn cmd_write_buffer_marker2_amd(
        &self,
        command_buffer: vk::CommandBuffer,
        stage: vk::PipelineStageFlags2KHR,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        marker: u32,
    ) {
        let fp = self
            .fp_cmd_write_buffer_marker2_amd
            .expect("vkCmdWriteBufferMarker2AMD is not loaded");
        (fp)(Some(command_buffer), stage, Some(dst_buffer), dst_offset, marker);
    }
    pub unsafe fn get_queue_checkpoint_data2_nv_to_vec(&self, queue: vk::Queue) -> Vec<vk::CheckpointData2NV> {
        let fp = self
            .fp_get_queue_checkpoint_data2_nv
            .expect("vkGetQueueCheckpointData2NV is not loaded");
        let mut len = MaybeUninit::<_>::uninit();
        (fp)(Some(queue), len.as_mut_ptr(), ptr::null_mut());
        let mut len = len.assume_init();
        let mut v = Vec::with_capacity(len as usize);
        (fp)(Some(queue), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_instance_version() {
        let loader = Loader::new().unwrap();
        let v = unsafe { loader.enumerate_instance_version() };
        assert!(v.is_ok());
    }
}
