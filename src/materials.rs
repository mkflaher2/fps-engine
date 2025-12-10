use macroquad::prelude::*;

pub fn default_world() -> Material {
    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };
    let material = load_material(
        ShaderSource::Glsl {
            vertex: DEFAULT_VERTEX,
            fragment: DEFAULT_FRAGMENT,
        },
        MaterialParams {
            pipeline_params: pipeline_params,
            ..Default::default()
        }
    ).unwrap();
    material
}

pub fn skybox() -> Material {
    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };
    let material = load_material(
        ShaderSource::Glsl {
            vertex: SKYBOX_VERTEX,
            fragment: SKYBOX_FRAGMENT,
        },
        MaterialParams {
            pipeline_params: pipeline_params,
            ..Default::default()
        }
    ).unwrap();
    material
}

const DEFAULT_VERTEX: &'static str = "#version 330 core

//layout (location=0) in vec3 aPos;
in vec3 position;
in vec2 texcoord;

out vec2 uv;
out vec3 fragPos;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
    fragPos = position;
}
";

const DEFAULT_FRAGMENT: &'static str = "#version 330 core
in vec2 uv;
in vec3 fragPos;

out vec4 fragColor;
void main() {

    //normal vector
    vec3 U = dFdx(fragPos);
    vec3 V = dFdy(fragPos);
    vec3 norm = normalize(cross(U, V));

    //
    vec3 lightPos = vec3(20.,10.,10.);
    vec3 lightDir = normalize(lightPos - fragPos);

    float dif = max((dot(norm, lightDir) + 1) / 2., 0.05);
    fragColor = vec4(abs(norm) * dif, 1.0);
}
";

const SKYBOX_VERTEX: &'static str = "#version 330 core

in vec3 position;
in vec2 texcoord;

out vec2 uv;
out vec3 fragPos;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
    fragPos = position;
}
";


const SKYBOX_FRAGMENT: &'static str = "#version 330 core

in vec2 uv;

uniform sampler2D Texture;

out vec4 fragColor;

void main() {
    fragColor = texture(Texture, uv);
}";
