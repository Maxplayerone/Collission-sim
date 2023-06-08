 #version 330 core
    out vec4 fragColor;

    uniform vec2 u_resolution;

    in vec3 v_pos;

    void main() {
        vec2 uv = vec2(v_pos.x * 2.0, v_pos.y * 2.0);

        float aspect = u_resolution.x / u_resolution.y;
        uv.x *= aspect;

        fragColor.rg = uv;
        fragColor.b = 0.0;
        
        float distance = 1.0 - length(uv); 
        distance = step(0.0, distance);

        fragColor.rgb = vec3(distance);       
        if(distance <= 0.0){
            fragColor.a = 0.0;    
        }else{
            fragColor.a = 1.0;
        }

    }