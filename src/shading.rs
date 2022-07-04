use crate::material;
use crate::light;
use crate::primatives;
use crate::color;

/* The Phong reflection model colors and object by adding together the
 * ambient room lighting, diffused light, and a specular highlight 
 * (reflection of light source)
 */
pub fn shade(mtrl: material::Material, lt: light::Light, loc: primatives::PointT, 
eyev: primatives::Vec3T, normalv: primatives::Vec3T) -> color::Color {


    //Calculate a vector from the light to the point being rendered
    let lightv = (lt.location - loc).normalized();

    //The color of the item will be brighter or dimmer based on 
    //the intesity of the light in the room
    let effecive_color = mtrl.color * lt.intensity;

    let ambient_color = effecive_color * mtrl.ambient;

    /* The dot product will represent the angle between the light vector
     * and the normal vector. If this angle is negative, the light is
     * on the other side of the surface, so no light will ever hit this
     * point
     */
    let light_normal_dot_prod = primatives::dot_product(lightv, normalv);

    let (diffuse, specular) = 
        if light_normal_dot_prod < 0.0 {
            (color::BLACK, color::BLACK)
        } else {
            let diffuse = effecive_color * mtrl.diffuse * light_normal_dot_prod;

            /* If the dot product of the reflected angle and the eye vector is
             * negative, then the specular highlight reflects away from the eye
             */
            let reflectv = normalv.reflect(-lightv);
            let reflect_eye_dot_prod = primatives::dot_product(reflectv, eyev.normalized());
            let specular =
                if reflect_eye_dot_prod <= 0.0 {
                    color::BLACK
                } else {
                    //diffuse = effecive_color * mtrl.diffuse * reflect_eye_dot_prod;
                    let factor = reflect_eye_dot_prod.powf(mtrl.shininess);
                    lt.intensity * mtrl.specular * factor
                };

            (diffuse, specular)
        };

        return ambient_color + diffuse + specular;
} 