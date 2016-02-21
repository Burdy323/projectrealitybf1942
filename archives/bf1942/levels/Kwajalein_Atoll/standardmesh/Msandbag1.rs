subshader "Msandbag1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.12549 0.12549 0.12549;
	materialSpecularPower 12.5;
	texture "texture/sandbag_o";
}

subshader "Msandbag1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	texture "texture/pabam2_s";
}

subshader "Msandbag1_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	twosided true;
	texture "texture/Greencamo";
}

