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

float sdSphere(vec3 p, float d) { return length(p) - d; } 

float sdBox( vec3 p, vec3 b )
{
  vec3 d = abs(p) - b;
  return min(max(d.x,max(d.y,d.z)),0.0) +
         length(max(d,0.0));
}
	
bool getVoxel(vec3 c) {
	vec3 p = c + vec3(0.5);
	// float d = max(-sdSphere(p, 5.5), sdBox(p, vec3(4.0)));
	float d = sdSphere(p, 2.5);
	return d < 0.0;
}


// vec4 getVoxel(ivec3 Position) {
// 	uint material = voxel_volume_data[uint(Position.z * voxel_volume_size.x * voxel_volume_size.y + Position.y * voxel_volume_size.x + Position.x)].material;
//     return voxel_volume_palette[material];
// }

void main(void) {

    vec3 RayOrigin = v_Near.xyz;
    vec3 RayDirection = v_Far.xyz - v_Near.xyz;

    mat4 InverseView = inverse(View) / 2.0;
    vec3 CameraPosition = vec3(InverseView[3]);
    vec3 RayPosition = (Model * vec4(v_Position, 1.0)).xyz;
    // RayDirection -= 0.000001;

	vec3 mapPos = vec3(floor(RayPosition + 0.));

	vec3 deltaDist = abs(vec3(length(RayDirection)) / RayDirection);
	
	vec3 rayStep = vec3(sign(RayDirection));

	vec3 sideDist = (sign(RayDirection) * (vec3(mapPos) - RayPosition) + (sign(RayDirection) * 0.5) + 0.5) * deltaDist; 
	
	bvec3 mask;
	
    vec4 color = vec4(0.0, 0.0, 0.0, 0.0);

	for (int i = 0; i < MAX_RAY_STEPS; i++) {        
        // color = getVoxel(mapPos);
        
		// if (color.a != 0.0)
        //     break;
        if (getVoxel(mapPos)) break;
        if (any(greaterThan(mapPos, voxel_volume_size - 1.0))) {
            mask = bvec3(false, false, false);
            break;
        }
        if (any(lessThan(mapPos, -(voxel_volume_size - 1.0)))) {
            mask = bvec3(false, false, false);
            break;
        }

		mask = lessThanEqual(sideDist.xyz, min(sideDist.yzx, sideDist.zxy));
        sideDist += vec3(mask) * deltaDist;
        mapPos += vec3(mask) * rayStep;
	}
	
	// if (mask.x) {
	// 	color *= vec4(vec3(0.5), 1.0);
	// }
	// if (mask.y) {
	// 	color *= vec4(vec3(1.0), 1.0);
	// }
	// if (mask.z) {
	// 	color *= vec4(vec3(0.75), 1.0);
	// }

    if (mask.x) {
		color = vec4(1.0, 0.0, 0.0, 1.0);
	}
	if (mask.y) {
		color = vec4(0.0, 1.0, 0.0, 1.0);
	}
	if (mask.z) {
		color = vec4(0.0, 0.0, 1.0, 1.0);
	}

	o_Target = color;
}
