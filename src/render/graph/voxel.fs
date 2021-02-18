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

    // vec3 Normal = mat3(Model) * v_Normal;

    vec3 OppositeForwardNormal = v_Normal * -1.0;
    vec3 OppositeRightNormal = normalize(cross(vec3(0.0, -1.0, 0.0), OppositeForwardNormal));
    vec3 OppositeUpNormal = normalize(cross(OppositeForwardNormal, OppositeForwardNormal));


    // Get the camera's position in 3D space based on the provided view matrix, and transform it into world space
    mat4 InverseView = inverse(View);
    vec3 CameraPosition = (Model * vec4(vec3(InverseView[3]), 0.)).xyz;

    // The current position of this fragment relative to the centre of the cube (local space), in meters
    vec3 BackFacePosition = v_Position;

    // The current position of this fragment in world space
    vec3 BackFaceModelPosition = (Model * vec4(BackFacePosition, 1.0)).xyz;

    // The direction from the camera to the fragment
    vec3 BackFaceRayDirection = normalize(BackFaceModelPosition - CameraPosition);

    // o_Target = vec4(vec3(BackFacePosition), 1.0);
    // return;

    // My best effort following the formula described here: https://stackoverflow.com/a/4248306/1427397
    // Finding t for the back face, then for each front face

    // t for point A (the back face, initial position) -- t_in in the StackOverflow answer
    // vec3 ta = (BackFacePosition - BackFacePosition * v_Normal) / -BackFaceRayDirection * -v_Normal;

    // t for opposing front faces -- components of the minimum function that defines t_out in the StackOverflow answer
    vec3 tbx = (OppositeRightPoint - BackFacePosition * OppositeRightNormal) / -BackFaceRayDirection * OppositeRightNormal;
    vec3 tby = (OppositeUpPoint - BackFacePosition * OppositeUpNormal) / -BackFaceRayDirection * OppositeUpNormal;
    vec3 tbz = (OppositeForwardPoint - BackFacePosition * OppositeForwardNormal) / -BackFaceRayDirection * OppositeForwardNormal;

    // minimum of tb components -- t_out in the SO answer
    vec3 tb = min(tbx, min(tby, tbz));

    o_Target = vec4(abs(tb), 1.0);
    return;

    // Distance between a and b in local space
    // vec3 len = max(max(tb, vec3(0.0)) - max(ta, vec3(0.0)), vec3(0.0));
    vec3 len = max(tb, vec3(0.0));

    o_Target = vec4(vec3(len), 1.0);
    return;
    
    // Move from the back face position in local space to the front face position by
    // following the ray back towards the camera for the defined distance
    vec3 FrontFacePosition = BackFacePosition + -BackFaceRayDirection * len;

    // o_Target = vec4(vec3(tb), 1.0);
    // return;

    // Get the front face position in world space
    vec3 FrontFaceModelPosition = (Model * vec4(FrontFacePosition, 1.0)).xyz;

    // o_Target = vec4(FrontFaceModelPosition, 1.0);
    // return;

    // Get the direction from the camera to the front face
    // (this is probably incomplete as it may need need to account for the normal of the front face plane)
    vec3 FrontFaceRayDirection = normalize(FrontFaceModelPosition - CameraPosition);

    // o_Target = vec4(FrontFaceRayDirection, 1.0);
    // return;

    // Convert the local space position into voxel space, ie. [-1, 1] -> [0, 32]
    vec3 ScaledPosition = ((FrontFacePosition + (scale / 2.0)) / scale) * voxel_volume_size;

    o_Target = vec4(floor(ScaledPosition) / 16.0, 1.0);
    return;

    // Set the ray direction for the ray marcher
    vec3 RayDirection = FrontFaceRayDirection;

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
