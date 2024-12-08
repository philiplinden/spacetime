# Goal Oriented Action Planning (GOAP)

A Finite State Machine (FSM) is a graph of states with discrete, pre-defined transitions between them. The path from the
current state to a goal state is deterministic according to the strict node graph of states and actions to transition
between them.

Goal Oriented Action Planning (GOAP)[^1] is similar to a FSM, except the rigid transitions are removed. Instead, the
algorithm composes a graph on-the-fly to transition from the current state to the goal state.

[^1]: Orkin, Jeff. "Three states and a plan: the AI of FEAR." _Game developers conference._ Vol. 2006. SanJose, California: CMP Game Group, 2006.

Every **Action** has a _Precondition_ and an _Effect_. The _Precondition_ is a condition that must be present in order
for the action to take place. The _Effect_ is how the action has changed the state or the environment after it has
occurred. A collection of actions is interchangable along these precondition-effect interfaces like a set of dominoes
laid out on a board. The preconditions can be matched with effects, or effects matched with preconditions, in any
arrangement that satisfies each action's requirements. Action "chains" are created by arranging compatible actions along their precondition-effect pairs until the current state has been changed into the goal state.

An agents with a collection of actions won't do anything until presented with a goal state. The planning stage in GOAP
builds a chain from the goal back to the current state by matching precondition-effect pairs. This chain of actions is
called a _plan_. The last action in the chain generates the effect that matches the goal state's preconditions. Actions
are prepended to one another until the current state (or "World state") is reached. If the plan is completed and the
current state isn't met by the end, that plan is abandoned. The algorithm repeats, using the next available permutation
of actions.

Since this can result in multiple valid plans, selecting one versus another is determined by assigning a cost value to
each action or effect. The plan with the lowest cost is the one that is selected. In the case of a tie in cost, the plan
can be chosen between them at random. This process of building and selecting plans uses the A* algorithm to find the
optimal path.

Once a plan is chosen, a simple FSM is created to move the actor to the next location, then perform the next action.
The agent works its way through the chain of actions (checking to see if the next action is still valid each time) until
a goal is reached.

The advantage of GOAP over a FSM presents itself when there are many possible actions or many goal states. With GOAP,
a plan can be quickly composed that represents the same graph of events as a very complicated FSM, and the GOAP graph
can change and adapt to changing conditions in the environment as it is executing a plan.
