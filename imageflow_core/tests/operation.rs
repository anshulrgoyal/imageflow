extern crate imageflow_core;

use imageflow_core::ffi::*;
use imageflow_types::*;
use imageflow_core::Context;
use criterion::{ criterion_group, criterion_main, Criterion};
use imageflow_core::graphics::bitmaps::*;


fn benchmark_transpose(ctx: &mut Criterion) {
    for w in (1u32..3000u32).step_by(1373) {
        for h in (1u32..3000u32).step_by(1373) {
            let c = Context::create().unwrap();
            let mut a = Bitmap::create_u8(w, h, PixelLayout::BGRA, true, true, ColorSpace::LinearRGB).unwrap();
            let mut a = unsafe {
                let mut a_bgra = a.get_window_u8().unwrap().to_bitmap_bgra(BitmapCompositingMode::ReplaceSelf).unwrap();
                a_bgra.fill_rect(&c, 0u32, 0u32, w, h, &Color::Srgb(ColorSrgb::Hex("FF0000FF".to_string()))).unwrap();
                a_bgra
            };
            let mut b = Bitmap::create_u8(h,w, PixelLayout::BGRA, true, true, ColorSpace::LinearRGB).unwrap();
            let mut b = unsafe {
                let a_bgra=b.get_window_u8().unwrap().to_bitmap_bgra(BitmapCompositingMode::ReplaceSelf).unwrap();
                a_bgra
            };
            ctx.bench_function(&format!("transpose w={} && h={}", w, h), |bn| bn.iter(|| { unsafe { assert_eq!(flow_bitmap_bgra_transpose(c.flow_c(), &mut a as *mut BitmapBgra, &mut b as *mut BitmapBgra), true) } }));
        }
    }
}



fn benchmark_flip_v(ctx: &mut Criterion) {
    let fmts=[PixelLayout::BGRA,PixelLayout::BGR];

    for &fmt in fmts.iter(){
        for w in (1u32..3000u32).step_by(1373){
            for h in (1u32..3000u32).step_by(1373){
                let c = Context::create().unwrap();
                let mut a = Bitmap::create_u8(w, h, fmt, true, true, ColorSpace::LinearRGB).unwrap();
                let mut a = unsafe {
                    let mut a_bgra = a.get_window_u8().unwrap().to_bitmap_bgra(BitmapCompositingMode::ReplaceSelf).unwrap();
                    a_bgra.fill_rect(&c, 0u32, 0u32, w, h, &Color::Srgb(ColorSrgb::Hex("FF0000FF".to_string()))).unwrap();
                    a_bgra
                };
                ctx.bench_function(&format!("flip_v w={} && h={} fmt={:?}",w,h,fmt), |bn| bn.iter(|| { unsafe { assert_eq!(flow_bitmap_bgra_flip_vertical(c.flow_c(), &mut a as *mut BitmapBgra),true) } } ));
            }
        }
    }

}


fn benchmark_flip_h(ctx: &mut Criterion) {
    let fmts=[PixelLayout::BGRA,PixelLayout::BGR];

    for &fmt in fmts.iter(){
        for w in (1u32..3000u32).step_by(1373){
            for h in (1u32..3000u32).step_by(1373){
                let c = Context::create().unwrap();
                let mut a = Bitmap::create_u8(w, h, fmt, true, true, ColorSpace::LinearRGB).unwrap();
                let mut a = unsafe {
                    let mut a_bgra = a.get_window_u8().unwrap().to_bitmap_bgra(BitmapCompositingMode::ReplaceSelf).unwrap();
                    a_bgra.fill_rect(&c, 0u32, 0u32, w, h, &Color::Srgb(ColorSrgb::Hex("FF0000FF".to_string()))).unwrap();
                    a_bgra
                };
                ctx.bench_function(&format!("flip_h w={} && h={} fmt={:?}",w,h,fmt), |bn| bn.iter(|| { unsafe { assert_eq!(flow_bitmap_bgra_flip_horizontal(c.flow_c(), &mut a as *mut BitmapBgra),true) } } ));
            }
        }
    }

}

fn benchmark_scale_2d(ctx: &mut Criterion) {
    let fmts=[PixelLayout::BGRA];
    let float_spaces=[Floatspace::Srgb,Floatspace::Linear];
    for &float_space in float_spaces.iter(){
        for &fmt in fmts.iter(){
            for w in (2000u32..4000u32).step_by(1373){
                for h in (2000u32..4000u32).step_by(1373){
                    let c = Context::create().unwrap();
                    let mut a = Bitmap::create_u8(w, h, PixelLayout::BGRA, true, true, ColorSpace::LinearRGB).unwrap();
                    let mut a = unsafe {
                        let mut a_bgra = a.get_window_u8().unwrap().to_bitmap_bgra(BitmapCompositingMode::ReplaceSelf).unwrap();
                        a_bgra.fill_rect(&c, 0u32, 0u32, w, h, &Color::Srgb(ColorSrgb::Hex("FF0000FF".to_string()))).unwrap();
                        a_bgra
                    };
                    let mut b = Bitmap::create_u8(800u32,800u32, PixelLayout::BGRA, true, true, ColorSpace::LinearRGB).unwrap();
                    let mut b = unsafe {
                        let a_bgra=b.get_window_u8().unwrap().to_bitmap_bgra(BitmapCompositingMode::ReplaceSelf).unwrap();
                        a_bgra
                    };
                    let scale=Scale2dRenderToCanvas1d{
                        x: 0u32,
                        y: 0u32,
                        w:800u32,
                        h:800u32,
                        sharpen_percent_goal: 0.0,
                        interpolation_filter: Filter::RobidouxFast,
                        scale_in_colorspace: float_space
                    };
                    ctx.bench_function(&format!("scale_2d w={} && h={} fmt={:?} float_space={:?}",w,h,fmt,float_space), |bn| bn.iter(|| {
                        unsafe { assert_eq!(flow_node_execute_scale2d_render1d(c.flow_c(), &mut a as *mut BitmapBgra,&mut b as *mut BitmapBgra,&scale),true) }
                    } ));
                }
            }
        }
    }
}

extern "C" {
    pub fn flow_scale_spatial_srgb_7x7(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_srgb_6x6(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_srgb_5x5(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_srgb_4x4(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_srgb_3x3(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_srgb_2x2(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_srgb_1x1(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_7x7(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_6x6(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_5x5(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_4x4(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_3x3(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_2x2(input:*const u8, output_rows:*const*mut u8, output_col:u32);
    pub fn flow_scale_spatial_1x1(input:*const u8, output_rows:*const*mut u8, output_col:u32);
}

fn benchmark_downscaling(ctx: &mut Criterion) {
    unsafe {
        let mut output =[{
            let mut temp=[0u8;8];
            temp.as_mut_ptr()
        };8];

        let ptr = output.as_mut_ptr();
        let mut input =[0u8;64];
        let input_ptr = input.as_ptr();
        let funs = [flow_scale_spatial_srgb_7x7,
            flow_scale_spatial_srgb_6x6,
            flow_scale_spatial_srgb_5x5,
            flow_scale_spatial_srgb_4x4,
            flow_scale_spatial_srgb_3x3, flow_scale_spatial_srgb_2x2,
            flow_scale_spatial_srgb_1x1, flow_scale_spatial_7x7, flow_scale_spatial_6x6,
            flow_scale_spatial_5x5, flow_scale_spatial_4x4, flow_scale_spatial_3x3,
            flow_scale_spatial_2x2, flow_scale_spatial_1x1
        ];


        for (i,&fun) in funs.iter().enumerate() {
            ctx.bench_function(&format!("downscale function={}",i), |bn| bn.iter(|| {
                fun(input_ptr, ptr, 0)
            }));
        }
    }
}

criterion_group!(benches, benchmark_transpose,benchmark_scale_2d,benchmark_downscaling,benchmark_flip_h,benchmark_flip_v);
criterion_main!(benches);

