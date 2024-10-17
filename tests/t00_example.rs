
#[test]
fn t00_example() {
    use agg::Render;

    // Create a blank image 10x10 pixels
    let pix = agg::Pixfmt::<agg::Rgb8>::new(100,100);
    let mut ren_base = agg::RenderingBase::new(pix);
    ren_base.clear(agg::Rgba8::white());

    // Draw a polygon from (10,10) - (50,90) - (90,10)
    let mut ras = agg::RasterizerScanline::new();
    ras.move_to(10.0, 10.0);
    ras.line_to(50.0, 90.0);
    ras.line_to(90.0, 10.0);

    // Render the line to the image
    let mut ren = agg::RenderingScanlineAASolid::with_base(&mut ren_base);
    ren.color(agg::Rgba8::black());
    agg::render_scanlines(&mut ras, &mut ren);

    // Save the image to a file
    // ren_base.to_file("tests/tmp/little_black_triangle.png").unwrap();
    // assert!(agg::ppm::img_diff("tests/tmp/little_black_triangle.png", "images/little_black_triangle.png").unwrap());
}
