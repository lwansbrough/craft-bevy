#version 330
//in int gl_VertexID;
out vec2 out_gl_TexCoord[gl_MaxTextureCoords];
//out vec4 gl_Position;
void main() {
    float X = -1.0 + float((gl_VertexID & 1) << 2);
    float Y = -1.0 + float((gl_VertexID & 2) << 1);
    out_gl_TexCoord[0].st = vec2((X + 1.0) * 0.5, (Y + 1.0) * 0.5);
    gl_Position = vec4(X, Y, 0, 1);
}
