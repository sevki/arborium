cbuffer PerFrame : register(b0) {
    float4x4 viewProj;
    float3 lightDir;
    float time;
};

struct VSInput {
    float3 position : POSITION;
    float3 normal : NORMAL;
    float2 uv : TEXCOORD0;
};

struct PSInput {
    float4 position : SV_POSITION;
    float3 normal : NORMAL;
    float2 uv : TEXCOORD0;
};

PSInput VSMain(VSInput input) {
    PSInput output;
    output.position = mul(viewProj, float4(input.position, 1.0));
    output.normal = input.normal;
    output.uv = input.uv;
    return output;
}
