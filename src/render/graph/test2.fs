#version 450

// Adapted from https://github.com/gpdaniels/Raymarcher which itself is an adaptation of https://github.com/ivl/Voxgrind

// precision highp float;

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec3 v_Uv;
layout(location = 3) in vec4 v_Near;
layout(location = 4) in vec4 v_Far;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
    mat4 View;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) uniform Time {
    double TimeElapsed;
};

layout(set = 2, binding = 1) uniform Resolution {
    float ScreenResolutionX;
    float ScreenResolutionY;
};

struct VoxelData {
    uint material;
};
layout(set = 3, binding = 0) buffer VoxelVolume {
    vec4 voxel_volume_palette[255];
    vec3 voxel_volume_size;
    VoxelData voxel_volume_data[];
};


const int MAX_RAY_STEPS = 64;

// float sdSphere(vec3 p, float d) { return length(p) - d; } 

// float sdBox( vec3 p, vec3 b )
// {
//   vec3 d = abs(p) - b;
//   return min(max(d.x,max(d.y,d.z)),0.0) +
//          length(max(d,0.0));
// }
	
// bool getVoxel(ivec3 c) {
// 	vec3 p = vec3(c) + vec3(0.5);
// 	float d = max(-sdSphere(p, 7.5), sdBox(p, vec3(6.0)));
// 	return d < 0.0;
// }


vec4 getVoxel(ivec3 Position) {
	uint material = voxel_volume_data[uint(Position.z * voxel_volume_size.x * voxel_volume_size.y + Position.y * voxel_volume_size.x + Position.x)].material;
    return voxel_volume_palette[material];
}

void main(void) {

    // o_Target = vec4(1.0, 0.0, 0.0, 1.0);
    // return;

    vec3 RayOrigin = v_Near.xyz;
    vec3 RayDirection = v_Far.xyz - v_Near.xyz;

    mat4 InverseView = inverse(View);
    vec3 CameraPosition = vec3(InverseView[3]);
    // vec3 FragPosition = v_Position + vec3(Model[3]);
    // vec3 RayDirection = normalize(RayOrigin - CameraPosition);
    // vec3 RayPosition = v_Position * voxel_volume_size;
    vec3 RayPosition = (Model * vec4(v_Position * voxel_volume_size, 1.0)).xyz;
    // RayDirection -= 0.000001;


    // vec3 ForwardVector = normalize(vec3(-InverseView[2]));
    // vec3 RightVector = normalize(cross(vec3(0.0, -1.0, 0.0), ForwardVector));
    // vec3 UpVector = normalize(cross(ForwardVector, RightVector));
    // vec3 FragPosition = RayPosition + vec3(Model[3]);
    // The position of ray, that will advance as we raymarch.
    
    // o_Target = vec4(Translation, 1.0);
    // return;
    // The direction in which to advance the ray position.
    // vec3 RayDirection = normalize(CameraPosition - FragPosition);
    // vec3 RayDirection = normalize(ForwardVector);
    // vec3 RayDirection = normalize(vec3(View[2] * inverse(Model)[2]));

    // o_Target = vec4(RayPosition, 1.0);
    // return;

	ivec3 mapPos = ivec3(floor(RayPosition + 0.));

    // o_Target = vec4(mapPos, 1.0);
    // return;

	vec3 deltaDist = abs(vec3(length(RayDirection)) / RayDirection);
	
	ivec3 rayStep = ivec3(sign(RayDirection));

	vec3 sideDist = (sign(RayDirection) * (vec3(mapPos) - RayPosition) + (sign(RayDirection) * 0.5) + 0.5) * deltaDist; 
	
	bvec3 mask;
	
    vec4 color;

	for (int i = 0; i < MAX_RAY_STEPS; i++) {
        // if (any(greaterThanEqual(abs(mapPos), ivec3(3, 3, 3)))) {
        //     break;
        // }
        
        color = getVoxel(mapPos);
        
		if (color.a != 0.0)
            break;
        // if (getVoxel(mapPos)) continue;

		mask = lessThanEqual(sideDist.xyz, min(sideDist.yzx, sideDist.zxy));
        sideDist += vec3(mask) * deltaDist;
        mapPos += ivec3(vec3(mask)) * rayStep;
	}
	
	if (mask.x) {
		color *= vec4(vec3(0.5), 1.0);
	}
	if (mask.y) {
		color *= vec4(vec3(1.0), 1.0);
	}
	if (mask.z) {
		color *= vec4(vec3(0.75), 1.0);
	}

    // if (mask.x) {
	// 	color = vec4(1.0, 0.0, 0.0, 1.0);
	// }
	// if (mask.y) {
	// 	color = vec4(0.0, 1.0, 0.0, 1.0);
	// }
	// if (mask.z) {
	// 	color = vec4(0.0, 0.0, 1.0, 1.0);
	// }

	o_Target = color;
}
