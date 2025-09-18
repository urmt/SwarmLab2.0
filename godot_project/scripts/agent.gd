extends Node2D

# Agent script controlling individual swarm robot behavior

var interpreter = null
var internal_model = {}
var velocity = Vector2.ZERO  # For physics learning (inertia with decay)

func _ready():
    # Initialize interpreter instance (via GDNative)
    interpreter = preload("res://weave_lang_interpreter.gdns").new()
    internal_model = {
        "light_threshold": 0.6,
        "wind_speed": 0.0
    }
    # Initialize qualic state via WeaveLang
    interpreter.execute("qualic set C_q = 0.5")
    interpreter.execute("qualic set F_q = 0.7")

func _process(delta):
    # Calculate light intensity based on distance to LightSource (no clues given)
    var light_source = get_node("/root/SwarmLab/Environment/LightSource")
    var distance = global_position.distance_to(light_source.position)
    var max_distance = 400.0  # Assuming 800x600 viewport
    var light_intensity = 1.0 - min(distance / max_distance, 1.0)  # Inverse distance falloff

    # Simulate wind_speed (emergent from environment, no clues)
    internal_model["wind_speed"] = sin(OS.get_ticks_msec() / 1000.0) * 0.5 + 0.5

    # Influence interpreter's model with wind (update in Rust)
    interpreter.update_wind_influence(internal_model["wind_speed"])

    # Prepare WeaveLang input string
    var input_string = "sense light_intensity = " + str(light_intensity) + 
                       " sense wind_speed = " + str(internal_model["wind_speed"])
    interpreter.execute(input_string)

    # Update agent behavior based on interpreter output
    if interpreter.should_move_forward():
        velocity += Vector2(cos(rotation), sin(rotation)) * 50 * delta
    elif interpreter.should_move_backward():
        velocity += Vector2(cos(rotation), sin(rotation)) * (-50) * delta

    # Physics learning: Apply velocity decay (friction) and update position
    velocity *= 0.95  # Learned decay factor (emergent from observation)
    position += velocity

    # Incorporate wind for orientation adjustment
    if interpreter.should_move_forward() or interpreter.should_move_backward():
        rotation += (internal_model["wind_speed"] - 0.5) * delta  # Turn with wind drift

    # Flocking behavior (simple alignment, emergent from swarm observation)
    align_with_flock()

func align_with_flock():
    # Placeholder for flocking: Align toward average position (learned from swarm)
    var avg_pos = Vector2.ZERO
    var agents = get_tree().get_nodes_in_group("agents")  # Assume agents in group
    for agent in agents:
        if agent != self:
            avg_pos += agent.position
    if agents.size() > 1:
        avg_pos /= (agents.size() - 1)
        var dir_to_center = (avg_pos - position).normalized()
        rotation = dir_to_center.angle() * 0.1 + rotation * 0.9  # Smooth alignment
