extern crate dxgi;
extern crate dxguid;
extern crate winapi;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use dxguid::IID_IDXGIFactory;
use dxgi::CreateDXGIFactory;

use winapi::dxgi::{IDXGIFactory, IDXGIAdapter, DXGI_ADAPTER_DESC};
use winapi::winerror::DXGI_ERROR_NOT_FOUND;

use std::io::prelude::*;
use std::ops::*;
use std::fs::File;
use std::mem::zeroed;
use std::ptr::null_mut;
use std::str::FromStr;
use std::borrow::*;
use std::clone::Clone;


#[derive(Serialize, Deserialize, Debug)]
pub struct GraphicsInfrastructureInfo {
    adapters: Vec<Adapter>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Adapter {
    index: u32,
    description: String,
    vendor_id: u32,
    device_id: u32,
    subsys_id: u32,
    revision: u32,
    bytes_video_memory:u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_query() {
        unsafe {query_graphics_infrastructure();}
    }
}

pub unsafe fn query_graphics_infrastructure() -> GraphicsInfrastructureInfo {
    
    let mut dxgi_factory = create_dxgi_factory();
    let adapter_list = query_dx_adapters(dxgi_factory.as_mut());
    
    let mut gfx_infra_info = GraphicsInfrastructureInfo{
        adapters: adapter_list,
    };

    let gfx_infra_info_json = serde_json::to_string_pretty(&gfx_infra_info).unwrap();
    let mut file: File = File::create("graphics_infra_information.txt").unwrap();
    write!(file, "{}", gfx_infra_info_json).unwrap();
    gfx_infra_info
}

pub unsafe fn create_dxgi_factory() -> Box<IDXGIFactory> {
    let mut factory_ptr = null_mut();
    CreateDXGIFactory(&IID_IDXGIFactory, &mut factory_ptr);
    Box::from_raw(factory_ptr as *mut IDXGIFactory)
}

pub unsafe fn query_dx_adapters(dxgi_factory: &mut IDXGIFactory) -> Vec<Adapter> {
    let mut adapter_vec = Vec::new();
    let mut adapter: *mut IDXGIAdapter = zeroed();
    let mut current_index:u32 = 0;
    while dxgi_factory.EnumAdapters(current_index, &mut adapter) != DXGI_ERROR_NOT_FOUND {
        let mut desc: DXGI_ADAPTER_DESC = zeroed();
        (*adapter).GetDesc(&mut desc);
        adapter_vec.push(create_adapter(desc, current_index));
        current_index += 1;
    }
    adapter_vec

}
fn create_adapter(desc: DXGI_ADAPTER_DESC, index: u32) -> Adapter {
    let desc_text = String::from_utf16_lossy(&desc.Description).trim_right_matches(0 as char).to_string(); 
    let adapter = Adapter {
            index: index,
            description: desc_text,
            vendor_id: desc.VectorId,
            device_id: desc.DeviceId,
            subsys_id: desc.SubSysId,
            revision: desc.Revision,
            bytes_video_memory: desc.DedicatedVideoMemory,
    };
    adapter
}
 