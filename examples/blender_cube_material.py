import json
import time
from pathlib import Path

import bpy


started = time.perf_counter()

bpy.ops.object.select_all(action="SELECT")
bpy.ops.object.delete()

bpy.ops.mesh.primitive_cube_add(size=2.0, location=(0, 0, 1))
cube = bpy.context.object
cube.name = "AgentControl_Cube"

mat = bpy.data.materials.new("AgentControl_ElectricBlue")
mat.use_nodes = True
mat.node_tree.nodes["Principled BSDF"].inputs["Base Color"].default_value = (0.05, 0.35, 1.0, 1.0)
mat.node_tree.nodes["Principled BSDF"].inputs["Roughness"].default_value = 0.38
cube.data.materials.append(mat)

bpy.ops.object.light_add(type="AREA", location=(3, -4, 6))
light = bpy.context.object
light.name = "AgentControl_KeyLight"
light.data.energy = 500
light.data.size = 4

bpy.ops.object.camera_add(location=(5, -6, 4), rotation=(1.1, 0, 0.72))
bpy.context.scene.camera = bpy.context.object

if "BLENDER_EEVEE_NEXT" in bpy.types.RenderSettings.bl_rna.properties["engine"].enum_items:
    bpy.context.scene.render.engine = "BLENDER_EEVEE_NEXT"
else:
    bpy.context.scene.render.engine = "BLENDER_EEVEE"
bpy.context.scene.world.color = (0.02, 0.02, 0.025)

blend_path = Path("/tmp/agent-linux-control-blender-cube.blend")
bpy.ops.wm.save_as_mainfile(filepath=str(blend_path))

elapsed_ms = round((time.perf_counter() - started) * 1000, 2)
print(json.dumps({
    "task": "blender_cube_material",
    "blend": str(blend_path),
    "elapsed_ms": elapsed_ms,
    "objects": sorted(obj.name for obj in bpy.context.scene.objects),
}))
