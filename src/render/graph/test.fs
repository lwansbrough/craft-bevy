#version 450

// Adapted from https://github.com/gpdaniels/Raymarcher which itself is an adaptation of https://github.com/ivl/Voxgrind

// precision highp float;

layout(location = 0) in vec2 v_Position;
layout(location = 1) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
    mat4 View;
};

layout(set = 1, binding = 0) uniform Time {
    double TimeElapsed;
};

layout(set = 1, binding = 1) uniform Resolution {
    float ScreenResolutionX;
    float ScreenResolutionY;
};

struct VoxelData {
    uint material;
};
layout(set = 2, binding = 0) buffer VoxelVolume {
    vec4 voxel_volume_palette[255];
    vec3 voxel_volume_size;
    VoxelData voxel_volume_data[];
};

vec3 LightPosition = vec3(0.0, 100.0, 0.0);
vec3 SceneOffset = vec3(0.0, 0.0, 0.0);
float FogDistance = 1000.0;

// Convert HSL (Hue Saturation Lightness) to RGB.
vec3 HSL2RGB(in vec3 HSL) {
    vec3 RGB = clamp(abs(mod(HSL.x * 6.0 + vec3(0.0, 4.0, 2.0), 6.0) - 3.0) - 1.0, 0.0, 1.0);
    return HSL.z + HSL.y * (RGB - 0.5) * (1.0 - abs(2.0 * HSL.z - 1.0));
}

// Sampling from the voxel volume.
vec4 SampleVolume(in vec3 Position) {
    // The voxel data structure (as in Voxel.hpp).
    // std::uint8_t Saturation : 2;
    // std::uint8_t Alpha : 3;
    // std::uint8_t Tint : 3;
    // std::uint8_t Hue : 4;
    // std::uint8_t Light : 4;
    // std::uint8_t State : 2;
    // std::uint8_t Temperature : 3;
    // std::uint8_t Direction : 3;
    // std::uint8_t Density : 2;
    // std::uint8_t Strength : 3;
    // std::uint8_t FillLevel : 3;


    // return vec4(1.0, 0.0, 0.0, 1.0);
    uint material = voxel_volume_data[uint(Position.z * voxel_volume_size.x * voxel_volume_size.y + Position.y * voxel_volume_size.x + Position.x)].material;
    return voxel_volume_palette[material];
    
    
    // uint Data = texelFetch(BinarySampler, ivec2(Position.x , (Position.y + voxel_volume_size.y * floor(Position.z))), 0).r;
    // uint SaturationValue = (Data >> uint(0)) & uint(0x3);
    // float Saturation = float(SaturationValue) / 3.0f;
    // uint AlphaValue = (Data >> uint(2)) & uint(0x7);
    // float Alpha = float(AlphaValue ) / 7.0f;
    // uint TintValue = (Data >> uint(2 + 3)) & uint(0x7);
    // vec3 Tint = vec3(float((TintValue & uint(0x4)) == uint(0x4)), float((TintValue & uint(0x2)) == uint(0x2)), float((TintValue & uint(0x1)) == uint(0x1)));
    // uint HueValue = (Data >> uint(2 + 3 + 3)) & uint(0xF);
    // float Hue = float(HueValue - uint(4)) / 11.0f;
    // bool GreyscaleHueEnabled = (HueValue < uint(4));
    // float Greyscale = float(HueValue) / 3.0f;
    // uint LightValue = (Data >> uint(2 + 3 + 3 + 4)) & uint(0xF);
    // float Light = float(LightValue) / 15.0f;
    // uint StateValue = (Data >> uint(2 + 3 + 3 + 4 + 4)) & uint(0x3);
    // bvec4 State = bvec4(StateValue == uint(0x3), StateValue == uint(0x2), StateValue == uint(0x1), StateValue == uint(0x0));
    // if (GreyscaleHueEnabled) {
    //     return vec4(vec3(Greyscale * Light), Alpha);
    // }
    // else {
    //     return vec4(HSL2RGB(vec3(Hue, Saturation, Light)), Alpha);
    // }
}

// Testing for ray intersection with a box.
bool RayBoxIntersect(in vec3 RayOrigin, in vec3 RayDirection, in vec3 BoxMin, in vec3 BoxMax, out float IntersectionDepth) {
    vec3 OriginToBoxMinimumVector = (BoxMin - RayOrigin) / RayDirection;
    vec3 OriginToBoxMaximumVector = (BoxMax - RayOrigin) / RayDirection;
    vec3 MaximumVector = max(OriginToBoxMaximumVector, OriginToBoxMinimumVector);
    vec3 MinimumVector = min(OriginToBoxMaximumVector, OriginToBoxMinimumVector);
    float BackIntersectionDepth = min(MaximumVector.x, min(MaximumVector.y, MaximumVector.z));
    IntersectionDepth = max(max(MinimumVector.x, 0.0), max(MinimumVector.y, MinimumVector.z));
    return BackIntersectionDepth > IntersectionDepth;
}

bool IsInsideBox(vec3 Position, vec3 BoxBottomLeft, vec3 BoxTopRight) {
    vec3 DimensionInside = step(BoxBottomLeft, Position) - step(BoxTopRight, Position);
    return DimensionInside.x * DimensionInside.y * DimensionInside.z > 0.5;
}

void main(void) {

    // o_Target = vec4(voxel_volume_size.x, 0.0, 0.0, 1.0);
    // return;

    // o_Target = voxel_volume_palette[2];
    // return;

    // Size of the scene
    mat4 InverseView = inverse(View);
    vec3 CameraPosition = vec3(InverseView[3]);
    // vec3 CameraPosition = vec3(0.0, 0.0, 20.0);

    float NearClip = 1.0;
    float FarClip = 1000.0;
    float FieldOfView = 3.1415 / 4.0;
    vec2 ScreenResolution = vec2(ScreenResolutionX, ScreenResolutionY);
    vec4 FogColor = vec4(0.9, 0.9, 0.9, 1.0);
    vec3 ForwardVector = normalize(vec3(-InverseView[2]));
    // vec3 ForwardVector = normalize(vec3(0.0, 0.0, -1.0));
    // vec3 RightVector = normalize(vec3(-View[0]));
    // vec3 UpVector = normalize(vec3(-View[1]));
    vec3 RightVector = normalize(cross(vec3(0.0, -1.0, 0.0), ForwardVector));
    vec3 UpVector = normalize(cross(ForwardVector, RightVector));
    vec2 ViewportPosition = gl_FragCoord.xy / ScreenResolution;

    // Abort if we're not in the rendering region of the frame buffer.
    if ((ViewportPosition.x > 1.0) || (ViewportPosition.y > 1.0)) {
        o_Target = FogColor;
        return;
    }

    // Viewport size.
    vec2 ViewportSize = vec2(
        2.0 * NearClip * tan(FieldOfView * 0.5),
        2.0 * NearClip * tan(FieldOfView * 0.5) * ScreenResolution.y / ScreenResolution.x
    );

    // Lower left point of viewport.
    vec3 ViewportOrigin = (CameraPosition + (ForwardVector * NearClip)) - (0.5 * ViewportSize.x * RightVector) - (0.5 * ViewportSize.y * UpVector);
    // The point on the viewport corresponding to this pixel.
    vec3 RayOrigin = ViewportOrigin + (ViewportPosition.x * ViewportSize.x * RightVector) + (ViewportPosition.y * ViewportSize.y * UpVector);
    // The position of ray, that will advance as we raymarch.
    vec3 RayPosition = floor(RayOrigin);
    // The direction in which to advance the ray position.
    vec3 RayDirection = normalize(RayOrigin - CameraPosition);
    // Prevent some artifacts.
    RayDirection += 0.000001;
    // The ray march origin may be further forward than the ray origin if we can jump forward to the volume.
    vec3 RayMarchOrigin = RayOrigin;

    // Test if the ray starts outside of the voxel volume.
    if (!IsInsideBox(RayOrigin, vec3(0.0, 0.0, 0.0), voxel_volume_size)) {
        // Initialize the marching inside the bounds.
        float IntersectionDepth;
        if (!RayBoxIntersect(RayOrigin, RayDirection, vec3(0.0, 0.0, 0.0), voxel_volume_size, IntersectionDepth)) {
            o_Target = FogColor;
            return;
        }
        RayMarchOrigin = RayOrigin + RayDirection * IntersectionDepth + RayDirection * 0.0001;
        RayPosition = floor(RayMarchOrigin);
    }

    // Set up the ray marching parameters.
    vec3 RayStep = sign(RayDirection);
    vec3 MaxTranslation = (((0.5 + RayPosition) + 0.5 * RayStep) - RayMarchOrigin) / RayDirection;
    vec3 DeltaTranslation = RayStep / RayDirection;

    // Initially set the colour to the fog colour.
    o_Target = vec4(FogColor.r, FogColor.g, FogColor.b, 0.0);

    // Ray marching loop.
    for (int Iteration = 0; Iteration < 2048; ++Iteration) {
    //     // Sample the volume at the current ray position.
        vec4 Voxel = SampleVolume(RayPosition);

        // Test if the voxel is empty.
        if (Voxel.a > 0.0) {
            // Calculate the intersection depth.
            float IntersectionDepth;
            if (!RayBoxIntersect(RayOrigin, RayDirection, RayPosition, RayPosition + vec3(1.0, 1.0, 1.0), IntersectionDepth)) {
                o_Target = mix(o_Target, FogColor, FogColor.a);
                return;
            }
            // Calculate the location of the intersection.
            vec3 IntersectionPosition = RayOrigin + RayDirection * IntersectionDepth;
            // Calculate the lighting direction.
            vec3 Direction = IntersectionPosition - (RayPosition + vec3(0.5, 0.5, 0.5));
            vec3 AbsoluteDirection = abs(Direction);
            vec3 NormalDirection;
            vec3 ConsecutiveDirectionRight;
            vec3 ConsecutiveDirectionUp;
            if ((AbsoluteDirection.y > AbsoluteDirection.x) && (AbsoluteDirection.y > AbsoluteDirection.z)) {
                NormalDirection = vec3(0, sign(Direction.y), 0);
                ConsecutiveDirectionRight = vec3(1, 0, 0);
                ConsecutiveDirectionUp = vec3(0, 0, 1);
            }
            else if (AbsoluteDirection.x > AbsoluteDirection.z) {
                NormalDirection = vec3(sign(Direction.x), 0.0, 0.0);
                ConsecutiveDirectionRight = vec3(0.0, 1.0, 0.0);
                ConsecutiveDirectionUp = vec3(0.0, 0.0, 1.0);
            }
            else {
                NormalDirection = vec3(0.0, 0.0, sign(Direction.z));
                ConsecutiveDirectionRight = vec3(1.0, 0.0, 0.0);
                ConsecutiveDirectionUp = vec3(0.0, 1.0, 0.0);
            }
            #if 1
                // Ambient occlusion.
                vec3 NormalBlock = RayPosition + 0.5 + NormalDirection;
                float AmbientOcclusion = 0.0;
                vec3 FractionalIntersectionPosition = fract(IntersectionPosition);
                float MagnitudeFromRight = dot(FractionalIntersectionPosition, ConsecutiveDirectionRight);
                float MagnitudeFromUp = dot(FractionalIntersectionPosition, ConsecutiveDirectionUp);
                AmbientOcclusion = max(AmbientOcclusion, SampleVolume(NormalBlock + ConsecutiveDirectionUp).w * MagnitudeFromUp);
                AmbientOcclusion = max(AmbientOcclusion, SampleVolume(NormalBlock - ConsecutiveDirectionUp).w * (1.0 - MagnitudeFromUp));
                AmbientOcclusion = max(AmbientOcclusion, SampleVolume(NormalBlock + ConsecutiveDirectionRight).w * MagnitudeFromRight);
                AmbientOcclusion = max(AmbientOcclusion, SampleVolume(NormalBlock - ConsecutiveDirectionRight).w * (1.0 - MagnitudeFromRight));
                AmbientOcclusion = max(AmbientOcclusion, SampleVolume(NormalBlock + ConsecutiveDirectionUp + ConsecutiveDirectionRight).w * min(MagnitudeFromUp, MagnitudeFromRight));
                AmbientOcclusion = max(AmbientOcclusion, SampleVolume(NormalBlock + ConsecutiveDirectionUp - ConsecutiveDirectionRight).w * min(MagnitudeFromUp, (1.0 - MagnitudeFromRight)));
                AmbientOcclusion = max(AmbientOcclusion, SampleVolume(NormalBlock - ConsecutiveDirectionUp + ConsecutiveDirectionRight).w * min((1.0 - MagnitudeFromUp), MagnitudeFromRight));
                AmbientOcclusion = max(AmbientOcclusion, SampleVolume(NormalBlock - ConsecutiveDirectionUp - ConsecutiveDirectionRight).w * min((1.0 - MagnitudeFromUp), (1.0 - MagnitudeFromRight)));
                AmbientOcclusion = max(0.0, min(1.0, AmbientOcclusion * 0.5));
            #else
                float AmbientOcclusion = 0.0;
            #endif
            // Point lighting.
            float PointLight = (1.0 - AmbientOcclusion) * min(1.0, max(0.0, dot(NormalDirection, normalize(LightPosition - IntersectionPosition))));
            // Fog colour.
            // float Fog = min(1.0, length(IntersectionPosition - CameraPosition) / FogDistance);
            float Fog = 0.0;
            // Mix this voxel with the point light.
            vec4 VoxelColor = mix(Voxel * PointLight, FogColor, Fog);
            // vec4 VoxelColor = Voxel;
            // Mix the lit voxel with the previously combined colours.
            o_Target = mix(o_Target, VoxelColor, (1.0 - o_Target.a) * Voxel.a);
            o_Target.a = min(1.0, o_Target.a + Voxel.a);
            // Test if the current colour transparancy is solid.
            if (o_Target.a >= 1.0) {
                // If it is then return the current colour as there is no point marching further.
                o_Target.a = 1.0;
                return;
            }
        }

        // Branchless advance.
        bvec3 RayAdvanceMask = lessThanEqual(MaxTranslation.xyz, min(MaxTranslation.yzx, MaxTranslation.zxy));
        MaxTranslation += vec3(RayAdvanceMask) * DeltaTranslation;
        RayPosition += ivec3(RayAdvanceMask) * RayStep;

        // Test within bounds.
        if ((RayPosition.x >= voxel_volume_size.x || RayPosition.x < 0.0)
            || (RayPosition.y >= voxel_volume_size.y || RayPosition.y < 0.0)
            || (RayPosition.z >= voxel_volume_size.z || RayPosition.z < 0.0)) {
            // If not within bounds return current colour.
            o_Target.a = 1.0;
            return;
        }
    }

    // If we have reached the maximum number of iterations, just return the current colour.
    o_Target.a = 1.0;
}
