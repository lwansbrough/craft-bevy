#version 450

// Adapted from https://github.com/gpdaniels/Raymarcher which itself is an adaptation of https://github.com/ivl/Voxgrind

// precision highp float;

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Uv;
layout(location = 2) in vec3 v_Normal;

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


const int MAX_RAY_STEPS = 2048;

vec4 getVoxel(ivec3 Position) {
	uint material = voxel_volume_data[uint(Position.z * voxel_volume_size.x * voxel_volume_size.y + Position.y * voxel_volume_size.x + Position.x)].material;
    return voxel_volume_palette[material];
}

void main(void) {

    // o_Target = vec4(1.0, 0.0, 0.0, 1.0);
    // return;

    float NearClip = 1.0;
    float FarClip = 1000.0;
    float FieldOfView = 3.1415 / 4.0;

    // Size of the scene
    mat4 InverseView = inverse(View);
    vec3 CameraPosition = vec3(InverseView[3]);
    vec2 ScreenResolution = vec2(ScreenResolutionX, ScreenResolutionY);
    vec3 ForwardVector = normalize(vec3(-InverseView[2]));
    vec3 RightVector = normalize(cross(vec3(0.0, -1.0, 0.0), ForwardVector));
    vec3 UpVector = normalize(cross(ForwardVector, RightVector));

    vec2 ViewportSize = vec2(
        2.0 * NearClip * tan(FieldOfView * 0.5),
        2.0 * NearClip * tan(FieldOfView * 0.5) * ScreenResolution.y / ScreenResolution.x
    );

    vec2 ViewportPosition = gl_FragCoord.xy / ScreenResolution;

    vec3 ViewportOrigin = (CameraPosition + (ForwardVector * NearClip)) - (0.5 * ViewportSize.x * RightVector) - (0.5 * ViewportSize.y * UpVector);
    

    // vec3 RayOrigin = ViewportOrigin + (ViewportPosition.x * ViewportSize.x * RightVector) + (ViewportPosition.y * ViewportSize.y * UpVector);
    vec3 RayOrigin = v_Position * vec3(Model[3]);
    // The position of ray, that will advance as we raymarch.
    // vec3 RayPosition = floor(RayOrigin);
    vec3 Scale = vec3(
        length(Model[0].xyz),
        length(Model[1].xyz),
        length(Model[2].xyz)
    );
    vec3 RayPosition = floor(v_Position * Scale);
    // vec3 RayPosition = floor(RayOrigin);
    // o_Target = vec4(RayPosition, 1.0);
    // return;
    // The direction in which to advance the ray position.
    // vec3 RayDirection = normalize(RayOrigin - CameraPosition);
    // vec3 RayDirection = normalize(RayOrigin - CameraPosition);
    // vec3 RayDirection = normalize(RayOrigin - CameraPosition);
    vec3 RayDirection = normalize(ForwardVector * vec3(inverse(Model)[2]));

    // o_Target = vec4(RayDirection, 1.0);
    // return;

	ivec3 mapPos = ivec3(floor(RayPosition + 0.));

	vec3 deltaDist = abs(vec3(length(RayDirection)) / RayDirection);
	
	ivec3 rayStep = ivec3(sign(RayDirection));

	vec3 sideDist = (sign(RayDirection) * (vec3(mapPos) - RayPosition) + (sign(RayDirection) * 0.5) + 0.5) * deltaDist; 
	
	bvec3 mask;
	
    vec4 color;

	for (int i = 0; i < MAX_RAY_STEPS; i++) {
        color = getVoxel(mapPos);
        
		if (color.r != 0.0 && color.g != 0.0 && color.b != 0.0 && color.a != 0.0)
            break;

		mask = lessThanEqual(sideDist.xyz, min(sideDist.yzx, sideDist.zxy));
        sideDist += vec3(mask) * deltaDist;
        mapPos += ivec3(vec3(mask)) * rayStep;
	}
	
	if (mask.x) {
		color = vec4(vec3(0.5), 1.0);
	}
	if (mask.y) {
		color = vec4(vec3(1.0), 1.0);
	}
	if (mask.z) {
		color = vec4(vec3(0.75), 1.0);
	}

	o_Target = color;
}
