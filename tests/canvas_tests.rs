#[cfg(test)]
use frog::canvas;
use frog::color;

#[test]
#[ignore]
fn test_canvas_creation() {
    let canvas = canvas::new(1080, 960);
    assert_eq!(canvas.width, 1080);
    assert_eq!(canvas.height, 960);
    assert_eq!(canvas.contents[0], color::new(0.0, 0.0, 0.0));

    canvas.antialiased(4);
}

#[test]
fn test_canvas_plot_and_read() {
    let mut canvas = canvas::new(100, 100);

    let red = color::new(1.0, 0.0, 0.0);
    let green = color::new(0.0, 1.0, 0.0);
    let blue = color::new(0.0, 0.0, 1.0);
    let white = color::new(1.0, 1.0, 1.0);

    canvas.plot(-1, -1, green);
    let read_out = canvas.read(-1, -1);
    assert_eq!(green, read_out);

    canvas.plot(1, 1, red);
    canvas.plot(1, -1, blue);
    canvas.plot(-1, 1, white);

    //canvas.write_to_ppm("coord_plane_test.ppm");
}

#[test]
#[should_panic]
fn test_plot_out_of_bounds() {
    let mut canvas = canvas::new(100, 100);
    let c1 = color::new(1.0, 0.0, 0.0);
    canvas.plot(50, 50, c1);
}