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


const int MAX_RAY_STEPS = 64;

// Get the voxel material at a position within the volume, returns a clear colour if the space is empty.
vec4 getVoxel(vec3 Position) {
    uint material = voxel_volume_data[uint(Position.x + voxel_volume_size.x * (Position.y + voxel_volume_size.z * Position.z))].material;
    return voxel_volume_palette[material];
}

void main(void) {
    // Set the scale for the voxels
    vec3 scale = voxel_volume_size / 16.0;

    mat4 InverseView = inverse(View);
    mat4 CameraToModel = Model * InverseView;

    // vec3 Camera_RayOrigin = vec3(InverseView[3]);
    vec3 Camera_RayOrigin = vec3(0.0, 0.0, 0.0);

    vec3 Model_BackFacePosition = v_Position;
    vec3 Model_RayOrigin = (CameraToModel * vec4(Camera_RayOrigin, 1.0)).xyz;
    vec3 Model_RayDirection = normalize(Model_BackFacePosition - Model_RayOrigin);

    vec3 Model_XN = vec3(-sign(Model_RayOrigin.x), 0.0, 0.0);
    vec3 Model_YN = vec3(0.0, -sign(Model_RayOrigin.y), 0.0);
    vec3 Model_ZN = vec3(0.0, 0.0, -sign(Model_RayOrigin.z));

    float Xd = -0.5 * scale.x;
    float Yd = -0.5 * scale.x;
    float Zd = -0.5 * scale.x;

    float Xt = -(dot(Model_RayOrigin, Model_XN) - Xd) / dot(Model_RayDirection, Model_XN);
    vec3 Model_PX = Model_RayOrigin + Xt * Model_RayDirection;

    float Yt = -(dot(Model_RayOrigin, Model_YN) - Yd) / dot(Model_RayDirection, Model_YN);
    vec3 Model_PY = Model_RayOrigin + Yt * Model_RayDirection;

    float Zt = -(dot(Model_RayOrigin, Model_ZN) - Zd) / dot(Model_RayDirection, Model_ZN);
    vec3 Model_PZ = Model_RayOrigin + Zt * Model_RayDirection;

    float Check_X = Xt * sign(floor(abs(Model_RayOrigin.x) * 2.0));
    float Check_Y = Yt * sign(floor(abs(Model_RayOrigin.y) * 2.0));
    float Check_Z = Zt * sign(floor(abs(Model_RayOrigin.z) * 2.0));

    vec3 best = Model_BackFacePosition;
    if (Check_X > 0.0 || Check_Y > 0.0 || Check_Z > 0.0)
    {
        best = Model_PX;
        float best_length = Check_X;
        if (Check_Y > best_length)
        {
            best = Model_PY;
            best_length = Check_Y;
        }
        if (Check_Z > best_length)
        {
            best = Model_PZ;
        }
    }

    vec3 center_offset = vec3(0.5, 0.5, 0.5) * scale;

    vec3 Model_FrontFacePosition = (best + center_offset);

    // Convert the local space position into voxel space, ie. [-1, 1] -> [0, 32]
    vec3 ScaledPosition = Model_FrontFacePosition * voxel_volume_size / scale;

    o_Target = vec4(floor(ScaledPosition) / voxel_volume_size, 1.0);
    return;

    // Set the ray direction for the ray marcher
    vec3 RayDirection = Model_RayDirection;

    // Do ray marching, starting at the front face position in voxel space
    vec3 RayPosition = ScaledPosition + 0.00001 * RayDirection;
	vec3 mapPos = floor(RayPosition);

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
