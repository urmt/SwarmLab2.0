extends Node2D

# Agent script controlling individual swarm robot behavior

var interpreter = null
var internal_model = {}

func _ready():
    # Initialize interpreter instance (via GDNative)
    interpreter = preload("res://weave_lang_interpreter.gdns").new()
    internal_model = {
        "light_threshold": 0.6,
        "wind_speed": 0.0
    }
    # Initialize qualic state
    interpreter.execute("qualic set C_q = 0.5")
    interpreter.execute("qualic set F_q = 0.7")

func _process(delta):
    # Calculate light intensity based on distance to LightSource
    var light_source = get_node("/root/SwarmLab/Environment/LightSource")
    var distance = global_position.distance_to(light_source.position)
    var max_distance = 400.0  # Assuming 800x600 viewport
    var light_intensity = 1.0 - min(distance / max_distance, 1.0)  # Inverse distance falloff

    # Prepare WeaveLang input string
    var input_string = "sense light_intensity = " + str(light_intensity) + 
                       " sense wind_speed = " + str(internal_model["wind_speed"])
    interpreter.execute(input_string)

    # Update behavior based on interpreter (assuming new methods)
    if interpreter.should_move_forward():
        position += Vector2(cos(rotation), sin(rotation)) * 50 * delta  # Move in orientation
    elif interpreter.should_move_backward():
        position -= Vector2(cos(rotation), sin(rotation)) * 50 * delta

    # Update wind speed (placeholder simulation)
    internal_model["wind_speed"] = sin(OS.get_ticks_msec() / 1000.0) * 0.5 + 0.5
