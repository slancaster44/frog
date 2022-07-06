#[cfg(test)]
use frog::color;

#[test]
fn test_color_creation() {
    let this_color = color::Color {
        red: -0.5,
        green: 0.3,
        blue: 1.7
    };

    assert_eq!(this_color.red, -0.5);
}

#[test]
fn test_color_eq() {
    let c1 = color::new(0.0, 0.0, 0.0);
    let c2 = color::new(0.0, 0.0, 0.0);

    assert_eq!(c1, c2);
}

#[test]
fn test_color_add() {
    let c1 = color::new(0.9, 0.5, 0.75);
    let c2 = color::new(0.7, 0.1, 0.25);

    let result = c1 + c2;
    let expected = color::new(1.6, 0.6, 1.0);

    assert_eq!(result, expected);
}

#[test]
fn test_color_sub() {
    let c1 = color::new(0.9, 0.5, 0.75);
    let c2 = color::new(0.7, 0.1, 0.25);

    let result = c1 - c2;
    let expected = color::new(0.2, 0.4, 0.5);

    assert_eq!(result, expected);
}

#[test]
fn test_color_scalar() {
    let scalar = 2.0;
    let c1 = color::new(0.2, 0.3, 0.4);

    let r1 = scalar * c1;
    let r2 = c1 * scalar;
    let expected = color::new(0.4, 0.6, 0.8);

    assert_eq!(r1, expected);
    assert_eq!(r2, expected);
}

#[test]
fn test_color_mul() {
    let c1 = color::new(1.0, 0.2, 0.4);
    let c2 = color::new(0.9, 1.0, 0.1);
    
    let result = c2 * c1;
    let expected = color::new(0.9, 0.2, 0.04);

    assert_eq!(result, expected);
}

#[test]
fn test_color_to_255() {
    let c1 = color::new(1.5, -2.0, 0.5);
    
    let result = c1.scaled_from_1_to_255();
    let expected = color::new(255.0, 0.0, 127.0);

    assert_eq!(result, expected);
}