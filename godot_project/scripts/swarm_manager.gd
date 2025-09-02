extends Node

# Manages the swarm of agents

var agents = []

func _ready():
    # Spawn agents in the environment
    for i in range(20):
        var agent_scene = preload("res://scenes/agent.tscn")
        var agent_instance = agent_scene.instance()
        add_child(agent_instance)
        agent_instance.position = Vector2(randf() * 500, randf() * 500)
        agents.append(agent_instance)

func _process(delta):
    # Update swarm-level logic if needed
    pass
