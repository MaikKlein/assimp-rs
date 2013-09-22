use std::libc::*;

// re-export constants
pub use consts::*;


pub struct aiTexture;
pub struct aiMaterial;
pub struct aiAnimation;
pub struct aiLight;
pub struct aiCamera;
struct aiAnimMesh;


pub struct aiString {
	/** Binary length of the string excluding the terminal 0. This is NOT the 
	 *  logical length of strings containing UTF-8 multibyte sequences! It's
	 *  the number of bytes from the beginning of the string to its end.*/
	length: size_t,

	/** String buffer. Size limit is MAXLEN */
	data: [ c_char, ..1024/*MAXLEN*/ ],
}


pub struct aiMatrix4x4 {
	a1: f32, a2: f32, a3: f32, a4: f32,
	b1: f32, b2: f32, b3: f32, b4: f32,
	c1: f32, c2: f32, c3: f32, c4: f32,
	d1: f32, d2: f32, d3: f32, d4: f32,
}

pub struct aiVector3D {
	x: f32, y: f32, z: f32
}

pub struct aiColor4D {
	r: f32, g: f32, b: f32, a: f32
}

pub struct aiNode {
	/** The name of the node. 
	 *
	 * The name might be empty (length of zero) but all nodes which 
	 * need to be accessed afterwards by bones or anims are usually named.
	 * Multiple nodes may have the same name, but nodes which are accessed
	 * by bones (see #aiBone and #aiMesh::mBones) *must* be unique.
	 * 
	 * Cameras and lights are assigned to a specific node name - if there
	 * are multiple nodes with this name, they're assigned to each of them.
	 * <br>
	 * There are no limitations regarding the characters contained in
	 * this text. You should be able to handle stuff like whitespace, tabs,
	 * linefeeds, quotation marks, ampersands, ... .
	 */
	mName: aiString,

	/** The transformation relative to the node's parent. */
	mTransformation: aiMatrix4x4,

	/** Parent node. NULL if this node is the root node. */
	mParent: *aiNode,

	/** The number of child nodes of this node. */
	mNumChildren: c_uint,

	/** The child nodes of this node. NULL if mNumChildren is 0. */
	mChildren: **aiNode,

	/** The number of meshes of this node. */
	mNumMeshes: c_uint,

	/** The meshes of this node. Each entry is an index into the mesh */
	mMeshes: *c_uint,
}


pub struct aiFace {
	// Number of indices defining this face. 
	// The maximum value for this member is #AI_MAX_FACE_INDICES.
	mNumIndices: c_uint,

	// Pointer to the indices array. Size of the array is given in numIndices.
	mIndices: *c_uint,
}


// ---------------------------------------------------------------------------
/** @brief A single influence of a bone on a vertex.
 */
pub struct aiVertexWeight {
	// Index of the vertex which is influenced by the bone.
	mVertexId: c_uint,

	// The strength of the influence in the range (0...1).
	// The influence from all bones at one vertex amounts to 1.
	mWeight: f32,
}


pub struct aiBone {
	// The name of the bone. 
	mName: aiString,

	// The number of vertices affected by this bone
	// The maximum value for this member is #AI_MAX_BONE_WEIGHTS.
	mNumWeights: c_uint,

	// The vertices affected by this bone
	mWeights: *aiVertexWeight,

	// Matrix that transforms from mesh space to bone space in bind pose
	mOffsetMatrix: aiMatrix4x4,
}

// ---------------------------------------------------------------------------
/** @brief A mesh represents a geometry or model with a single material. 
*
* It usually consists of a number of vertices and a series of primitives/faces 
* referencing the vertices. In addition there might be a series of bones, each 
* of them addressing a number of vertices with a certain weight. Vertex data 
* is presented in channels with each channel containing a single per-vertex 
* information such as a set of texture coords or a normal vector.
* If a data pointer is non-null, the corresponding data stream is present.
* From C++-programs you can also use the comfort functions Has*() to
* test for the presence of various data streams.
*
* A Mesh uses only a single material which is referenced by a material ID.
* @note The mPositions member is usually not optional. However, vertex positions 
* *could* be missing if the #AI_SCENE_FLAGS_INCOMPLETE flag is set in 
* @code
* aiScene::mFlags
* @endcode
*/
struct aiMesh
{
	/** Bitwise combination of the members of the #aiPrimitiveType enum.
	 * This specifies which types of primitives are present in the mesh.
	 * The "SortByPrimitiveType"-Step can be used to make sure the 
	 * output meshes consist of one primitive type each.
	 */
	mPrimitiveTypes: c_uint,

	/** The number of vertices in this mesh. 
	* This is also the size of all of the per-vertex data arrays.
	* The maximum value for this member is #AI_MAX_VERTICES.
	*/
	mNumVertices: c_uint,

	/** The number of primitives (triangles, polygons, lines) in this  mesh. 
	* This is also the size of the mFaces array.
	* The maximum value for this member is #AI_MAX_FACES.
	*/
	mNumFaces: c_uint,

	/** Vertex positions. 
	* This array is always present in a mesh. The array is 
	* mNumVertices in size. 
	*/
	mVertices: *aiVector3D,

	/** Vertex normals. 
	* The array contains normalized vectors, NULL if not present. 
	* The array is mNumVertices in size. Normals are undefined for
	* point and line primitives. A mesh consisting of points and
	* lines only may not have normal vectors. Meshes with mixed
	* primitive types (i.e. lines and triangles) may have normals,
	* but the normals for vertices that are only referenced by
	* point or line primitives are undefined and set to QNaN (WARN:
	* qNaN compares to inequal to *everything*, even to qNaN itself.
	* Using code like this to check whether a field is qnan is:
	* @code
	* #define IS_QNAN(f) (f != f)
	* @endcode
	* still dangerous because even 1.f == 1.f could evaluate to false! (
	* remember the subtleties of IEEE754 artithmetics). Use stuff like
	* @c fpclassify instead.
	* @note Normal vectors computed by Assimp are always unit-length.
	* However, this needn't apply for normals that have been taken
	*   directly from the model file.
	*/
	mNormals: *aiVector3D,

	/** Vertex tangents. 
	* The tangent of a vertex points in the direction of the positive 
	* X texture axis. The array contains normalized vectors, NULL if
	* not present. The array is mNumVertices in size. A mesh consisting 
	* of points and lines only may not have normal vectors. Meshes with 
	* mixed primitive types (i.e. lines and triangles) may have 
	* normals, but the normals for vertices that are only referenced by
	* point or line primitives are undefined and set to qNaN.  See
	* the #mNormals member for a detailled discussion of qNaNs.
	* @note If the mesh contains tangents, it automatically also 
	* contains bitangents.
	*/
	mTangents: *aiVector3D,

	/** Vertex bitangents. 
	* The bitangent of a vertex points in the direction of the positive 
	* Y texture axis. The array contains normalized vectors, NULL if not
	* present. The array is mNumVertices in size. 
	* @note If the mesh contains tangents, it automatically also contains
	* bitangents.  
	*/
	mBitangents: *aiVector3D,

	/** Vertex color sets. 
	* A mesh may contain 0 to #AI_MAX_NUMBER_OF_COLOR_SETS vertex 
	* colors per vertex. NULL if not present. Each array is
	* mNumVertices in size if present.
	*/
	mColors: [ *aiColor4D, .. AI_MAX_NUMBER_OF_COLOR_SETS ],

	/** Vertex texture coords, also known as UV channels.
	* A mesh may contain 0 to AI_MAX_NUMBER_OF_TEXTURECOORDS per
	* vertex. NULL if not present. The array is mNumVertices in size. 
	*/
	mTextureCoords: [ *aiVector3D, .. AI_MAX_NUMBER_OF_TEXTURECOORDS ],

	/** Specifies the number of components for a given UV channel.
	* Up to three channels are supported (UVW, for accessing volume
	* or cube maps). If the value is 2 for a given channel n, the
	* component p.z of mTextureCoords[n][p] is set to 0.0f.
	* If the value is 1 for a given channel, p.y is set to 0.0f, too.
	* @note 4D coords are not supported 
	*/
	mNumUVComponents: [ c_uint, .. AI_MAX_NUMBER_OF_TEXTURECOORDS ],

	/** The faces the mesh is constructed from. 
	* Each face refers to a number of vertices by their indices. 
	* This array is always present in a mesh, its size is given 
	* in mNumFaces. If the #AI_SCENE_FLAGS_NON_VERBOSE_FORMAT
	* is NOT set each face references an unique set of vertices.
	*/
	mFaces: *aiFace,

	/** The number of bones this mesh contains. 
	* Can be 0, in which case the mBones array is NULL. 
	*/
	mNumBones: c_uint,

	/** The bones of this mesh. 
	* A bone consists of a name by which it can be found in the
	* frame hierarchy and a set of vertex weights.
	*/
	mBones: **aiBone,

	/** The material used by this mesh. 
	 * A mesh does use only a single material. If an imported model uses
	 * multiple materials, the import splits up the mesh. Use this value 
	 * as index into the scene's material list.
	 */
	mMaterialIndex: c_uint,

	/** Name of the mesh. Meshes can be named, but this is not a
	 *  requirement and leaving this field empty is totally fine.
	 *  There are mainly three uses for mesh names: 
	 *   - some formats name nodes and meshes independently.
	 *   - importers tend to split meshes up to meet the
	 *      one-material-per-mesh requirement. Assigning
	 *      the same (dummy) name to each of the result meshes
	 *      aids the caller at recovering the original mesh
	 *      partitioning.
	 *   - Vertex animations refer to meshes by their names.
	 **/
	mName: aiString,


	/** NOT CURRENTLY IN USE. The number of attachment meshes */
	mNumAnimMeshes: c_uint,

	/** NOT CURRENTLY IN USE. Attachment meshes for this mesh, for vertex-based animation. 
	 *  Attachment meshes carry replacement data for some of the
	 *  mesh'es vertex components (usually positions, normals). */
	mAnimMeshes: **aiAnimMesh,
}

pub struct aiScene {

	/** Any combination of the AI_SCENE_FLAGS_XXX flags. By default 
	* this value is 0, no flags are set. Most applications will
	* want to reject all scenes with the AI_SCENE_FLAGS_INCOMPLETE 
	* bit set.
	*/
	mFlags: c_uint,


	/** The root node of the hierarchy. 
	* 
	* There will always be at least the root node if the import
	* was successful (and no special flags have been set). 
	* Presence of further nodes depends on the format and content 
	* of the imported file.
	*/
	mRootNode: *aiNode,



	/** The number of meshes in the scene. */
	mNumMeshes: c_uint,

	/** The array of meshes. 
	*
	* Use the indices given in the aiNode structure to access 
	* this array. The array is mNumMeshes in size. If the
	* AI_SCENE_FLAGS_INCOMPLETE flag is not set there will always 
	* be at least ONE material.
	*/
	mMeshes: **aiMesh,



	/** The number of materials in the scene. */
	mNumMaterials: c_uint,

	/** The array of materials. 
	* 
	* Use the index given in each aiMesh structure to access this
	* array. The array is mNumMaterials in size. If the
	* AI_SCENE_FLAGS_INCOMPLETE flag is not set there will always 
	* be at least ONE material.
	*/
	mMaterials: **aiMaterial,



	/** The number of animations in the scene. */
	mNumAnimations: c_uint,

	/** The array of animations. 
	*
	* All animations imported from the given file are listed here.
	* The array is mNumAnimations in size.
	*/
	mAnimations: **aiAnimation,



	/** The number of textures embedded into the file */
	mNumTextures: c_uint,

	/** The array of embedded textures.
	* 
	* Not many file formats embed their textures into the file.
	* An example is Quake's MDL format (which is also used by
	* some GameStudio versions)
	*/
	mTextures: **aiTexture,


	/** The number of light sources in the scene. Light sources
	* are fully optional, in most cases this attribute will be 0 
        */
	mNumLights: c_uint,

	/** The array of light sources.
	* 
	* All light sources imported from the given file are
	* listed here. The array is mNumLights in size.
	*/
	mLights: **aiLight,


	/** The number of cameras in the scene. Cameras
	* are fully optional, in most cases this attribute will be 0 
        */
	mNumCameras: c_uint,

	/** The array of cameras.
	* 
	* All cameras imported from the given file are listed here.
	* The array is mNumCameras in size. The first camera in the
	* array (if existing) is the default camera view into
	* the scene.
	*/
	mCameras: **aiCamera,

	mPrivate: *c_void
}

// C function bindings

extern "C" {
	#[no_mangle] pub fn aiImportFile( pFile : *c_char, pFlags: c_uint ) -> *aiScene;
}
