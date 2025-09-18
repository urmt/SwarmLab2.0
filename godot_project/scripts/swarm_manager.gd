extends Node

# Manages the swarm of agents, coordinating their behavior and environment

export(int) var swarm_size = 20
export(float) var spawn_radius = 200.0
export(NodePath) var light_source_path

var agents = []
var wind_speed = 0.0

func _ready():
    # Initialize swarm
    for i in range(swarm_size):
        var agent = preload("res://agent.tscn").instance()
        agent.position = Vector2(rand_range(-spawn_radius, spawn_radius), rand_range(-spawn_radius, spawn_radius))
        agent.rotation = rand_range(0, 2 * PI)
        add_child(agent)
        agents.append(agent)
        agent.add_to_group("agents")
    
    # Set up light source reference
    if light_source_path:
        var light_source = get_node(light_source_path)
        if light_source:
            light_source.connect("moved", self, "_on_light_source_moved")

func _process(delta):
    # Simulate dynamic wind (emergent environmental factor)
    wind_speed = sin(OS.get_ticks_msec() / 1000.0) * 0.5 + 0.5
    update_agent_wind()

    # Manage flocking and physics coherence
    update_flocking()

func update_agent_wind():
    # Propagate wind speed to all agents
    for agent in agents:
        agent.internal_model["wind_speed"] = wind_speed
        if agent.interpreter:
            agent.interpreter.update_wind_influence(wind_speed)

func update_flocking():
    # Calculate average position and velocity for flocking
    var avg_pos = Vector2.ZERO
    var avg_vel = Vector2.ZERO
    for agent in agents:
        avg_pos += agent.position
        avg_vel += agent.velocity
    if agents.size() > 0:
        avg_pos /= agents.size()
        avg_vel /= agents.size()

    # Apply flocking rules (alignment, cohesion, separation)
    for agent in agents:
        var to_avg_pos = (avg_pos - agent.position).normalized() * 0.01
        var to_avg_vel = avg_vel.normalized() * 0.02
        agent.position += to_avg_pos
        agent.velocity += to_avg_vel
        # Simple separation (avoid crowding)
        for other in agents:
            if other != agent and agent.position.distance_to(other.position) < 20:
                agent.position -= (other.position - agent.position).normalized() * 0.05

func _on_light_source_moved(new_position):
    # Update light source position if it moves
    for agent in agents:
        agent.update_light_source(new_position)
