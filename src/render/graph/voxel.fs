#version 330
//in vec2 gl_FragCoord;
out vec4 out_gl_FragColor;
uniform vec2 ScreenResolution;
uniform vec2 FramebufferResolution;
uniform vec3 SceneOffset;
uniform vec3 LightPosition;
//uniform vec3 LightColour;
uniform vec3 CameraPosition;
uniform vec3 CameraTarget;
uniform float NearClip;
uniform float FieldOfView;
uniform float FogDistance;
uniform vec4 FogColour;
uniform vec3 VolumeSize;
// This sampler will get 32 bits of data for each voxel.
uniform usampler2D BinarySampler;
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
    uint Data = texelFetch(BinarySampler, ivec2(Position.x , (Position.y + VolumeSize.y * floor(Position.z))), 0).r;
    uint SaturationValue = (Data >> uint(0)) & uint(0x3);
    float Saturation = float(SaturationValue) / 3.0f;
    uint AlphaValue = (Data >> uint(2)) & uint(0x7);
    float Alpha = float(AlphaValue ) / 7.0f;
    uint TintValue = (Data >> uint(2 + 3)) & uint(0x7);
    vec3 Tint = vec3(float((TintValue & uint(0x4)) == uint(0x4)), float((TintValue & uint(0x2)) == uint(0x2)), float((TintValue & uint(0x1)) == uint(0x1)));
    uint HueValue = (Data >> uint(2 + 3 + 3)) & uint(0xF);
    float Hue = float(HueValue - uint(4)) / 11.0f;
    bool GreyscaleHueEnabled = (HueValue < uint(4));
    float Greyscale = float(HueValue) / 3.0f;
    uint LightValue = (Data >> uint(2 + 3 + 3 + 4)) & uint(0xF);
    float Light = float(LightValue) / 15.0f;
    uint StateValue = (Data >> uint(2 + 3 + 3 + 4 + 4)) & uint(0x3);
    bvec4 State = bvec4(StateValue == uint(0x3), StateValue == uint(0x2), StateValue == uint(0x1), StateValue == uint(0x0));
    if (GreyscaleHueEnabled) {
        return vec4(vec3(Greyscale * Light), Alpha);
    }
    else {
        return vec4(HSL2RGB(vec3(Hue, Saturation, Light)), Alpha);
    }
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
// Hash function to create "random" data from a seed.
float Hash(float Seed) {
    return fract(sin(Seed) * 43758.5453);
}
// Noise function, using hash function to create 3D "random" noise.
// The noise function returns a value in the range -1.0f -> 1.0f.
float Noise(vec3 Seed) {
    vec3 FloorSeed = floor(Seed);
    vec3 FractSeed = fract(Seed);
    FractSeed = FractSeed * FractSeed * (3.0 - 2.0 * FractSeed);
    float BaseSeed = FloorSeed.x + FloorSeed.y * 57.0 + 113.0 * FloorSeed.z;
    return mix(mix(mix(Hash(BaseSeed + 0.0), Hash(BaseSeed + 1.0), FractSeed.x),
                mix(Hash(BaseSeed + 57.0), Hash(BaseSeed + 58.0), FractSeed.x), FractSeed.y),
                mix(mix(Hash(BaseSeed + 113.0), Hash(BaseSeed + 114.0), FractSeed.x),
                mix(Hash(BaseSeed + 170.0), Hash(BaseSeed + 171.0), FractSeed.x), FractSeed.y), FractSeed.z);
}
// Main function.
void main(void) {
    // Calculate direction bectors from camera and target.
    vec3 ForwardVector = normalize(CameraTarget - CameraPosition);
    vec3 RightVector = normalize(cross(vec3(0.0, 1.0, 0.0), ForwardVector));
    vec3 UpVector = normalize(cross(ForwardVector, RightVector));
    // Calculate the fragment position on the viewport.
    vec2 ViewportPosition = gl_FragCoord.xy / ScreenResolution;
    // Abort if we're not in the rendering region of the frame buffer.
    if ((ViewportPosition.x > 1.0) || (ViewportPosition.y > 1.0)) {
        out_gl_FragColor = FogColour;
        return;
    }
    // Viewport size.
    vec2 ViewportSize = vec2(
        2.0 * NearClip * tan(radians(FieldOfView * 0.5)),
        2.0 * NearClip * tan(radians(FieldOfView * 0.5)) * ScreenResolution.y / ScreenResolution.x
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
    if (!IsInsideBox(RayOrigin, vec3(0.0, 0.0, 0.0), VolumeSize)) {
        // Initialize the marching inside the bounds.
        float IntersectionDepth;
        if (!RayBoxIntersect(RayOrigin, RayDirection, vec3(0.0, 0.0, 0.0), VolumeSize, IntersectionDepth)) {
            out_gl_FragColor = FogColour;
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
    out_gl_FragColor = vec4(FogColour.r, FogColour.g, FogColour.b, 0.0);
    // Ray marching loop.
    for (int Iteration = 0; Iteration < 2048; ++Iteration) {
        // Sample the volume at the current ray position.
        vec4 Voxel = SampleVolume(RayPosition);
        // Test if the voxel is empty.
        if (Voxel.a > 0.0) {
            // Calculate the intersection depth.
            float IntersectionDepth;
            if (!RayBoxIntersect(RayOrigin, RayDirection, RayPosition, RayPosition + vec3(1.0, 1.0, 1.0), IntersectionDepth)) {
                out_gl_FragColor = mix(out_gl_FragColor, FogColour, FogColour.a);
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
            float Fog = min(1.0, length(IntersectionPosition - CameraPosition) / FogDistance);
            // Apply some noise to the colour of this voxel.
            float ColourNoise = 0.3 * Noise(RayPosition.xyz + SceneOffset.xyz);
            Voxel.r += ColourNoise;
            Voxel.g += ColourNoise;
            Voxel.b += ColourNoise;
            // Mix this voxel with the point light.
            vec4 VoxelColour = mix(Voxel * PointLight, FogColour, Fog);
            // Mix the lit voxel with the previously combined colours.
            out_gl_FragColor = mix(out_gl_FragColor, VoxelColour, (1.0 - out_gl_FragColor.a) * Voxel.a);
            out_gl_FragColor.a = min(1.0, out_gl_FragColor.a + Voxel.a);
            // Test if the current colour transparancy is solid.
            if (out_gl_FragColor.a >= 1.0) {
                // If it is then return the current colour as there is no point marching further.
                out_gl_FragColor.a = 1.0;
                return;
            }
        }
        // Branchless advance.
        bvec3 RayAdvanceMask = lessThanEqual(MaxTranslation.xyz, min(MaxTranslation.yzx, MaxTranslation.zxy));
        MaxTranslation += vec3(RayAdvanceMask) * DeltaTranslation;
        RayPosition += ivec3(RayAdvanceMask) * RayStep;
        // Test within bounds.
        if ((RayPosition.x >= VolumeSize.x || RayPosition.x < 0.0)
            || (RayPosition.y >= VolumeSize.y || RayPosition.y < 0.0)
            || (RayPosition.z >= VolumeSize.z || RayPosition.z < 0.0)) {
            // If not within bounds return current colour.
            out_gl_FragColor.a = 1.0;
            return;
        }
    }
    // If we have reached the maximum number of iterations, just return the current colour.
    out_gl_FragColor.a = 1.0;
}
