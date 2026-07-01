import re

with open('src/backend/shaders.rs', 'r') as f:
    content = f.read()

content = content.replace('var<uniform> intensity: f32;', 'var<uniform> params: vec4<f32>;')
content = content.replace(' intensity', ' params.x')
content = content.replace('(intensity', '(params.x')
content = content.replace('intensity *', 'params.x *')

with open('src/backend/shaders.rs', 'w') as f:
    f.write(content)
