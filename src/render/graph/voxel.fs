#version 450

// precision highp float;

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec3 v_Uv;

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

// Volumes occupy a space of 1 meter per 16 voxels.
layout(set = 3, binding = 0) buffer VoxelVolume {
    vec4 voxel_volume_palette[256];
    vec3 voxel_volume_size;
    VoxelData voxel_volume_data[];
};


const int MAX_RAY_STEPS = 2048;

// Get the voxel material at a position within the volume, returns a clear colour if the space is empty.
vec4 getVoxel(vec3 Position) {
    uint material = voxel_volume_data[uint(Position.x + voxel_volume_size.x * (Position.y + voxel_volume_size.z * Position.z))].material;
    return voxel_volume_palette[material];
}

void main(void) {
    // Set the scale for the voxels
    vec3 scale = voxel_volume_size / 16.0;

    // mat4 InverseView = inverse(View);
    // mat4 CameraToModel = Model * InverseView;
    mat4 CameraToModel = inverse(Model) * inverse(View);

    // vec3 Camera_RayOrigin = vec3(InverseView[3]);
    vec3 Camera_RayOrigin = vec3(0.0, 0.0, 0.0);
    vec3 Model_BackFacePosition = v_Position;
    vec3 Model_RayOrigin = (CameraToModel * vec4(Camera_RayOrigin, 1.0)).xyz;
    vec3 Model_RayDirection = normalize(Model_BackFacePosition - Model_RayOrigin);

    vec3 center_offset = vec3(0.5, 0.5, 0.5) * scale;

    vec3 Model_N = -sign(Model_RayOrigin);
    vec3 d = -center_offset;
    vec3 t = -(Model_RayOrigin * Model_N - d) / (Model_RayDirection * Model_N);
    vec3 f = sign(floor(abs(Model_RayOrigin) * 2.0 / scale));
    float best_t = max(max(t.x * f.x, t.y * f.y), t.z * f.z);
    vec3 best = Model_BackFacePosition;
    if (f.x > 0.0 || f.y > 0.0 || f.z > 0.0)
    {
        best = Model_RayOrigin + best_t * Model_RayDirection;
    }

    vec3 Model_FrontFacePosition = (best + center_offset);

    // Convert the local space position into voxel space, ie. [-1, 1] -> [0, 32]
    vec3 ScaledPosition = Model_FrontFacePosition * voxel_volume_size / scale;

    // Set the ray direction for the ray marcher
    vec3 RayDirection = Model_RayDirection;

    // Do ray marching, starting at the front face position in voxel space
    vec3 RayPosition = ScaledPosition + 0.00001 * RayDirection;
	vec3 mapPos = floor(RayPosition);

    // o_Target = vec4(mapPos / voxel_volume_size, 1.0);
    // return;

	vec3 deltaDist = abs(vec3(length(RayDirection)) / RayDirection);
	
	vec3 rayStep = vec3(sign(RayDirection));

	vec3 sideDist = (sign(RayDirection) * (vec3(mapPos) - RayPosition) + (sign(RayDirection) * 0.5) + 0.5) * deltaDist; 
	
	bvec3 mask;
	
    vec4 color;

	for (int i = 0; i < MAX_RAY_STEPS; i++) {
        if (any(greaterThanEqual(mapPos, voxel_volume_size))) {
            color = vec4(0.0, 0.0, 0.0, 0.0);
            break;
        }
        if (any(lessThan(mapPos, vec3(0.0)))) {
            color = vec4(0.0, 0.0, 0.0, 0.0);
            break;
        }

        color = getVoxel(mapPos);
        
		if (color.a != 0.0)
            break;

		mask = lessThanEqual(sideDist.xyz, min(sideDist.yzx, sideDist.zxy));
        sideDist += vec3(mask) * deltaDist;
        mapPos += vec3(mask) * rayStep;
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

	o_Target = color;
}
