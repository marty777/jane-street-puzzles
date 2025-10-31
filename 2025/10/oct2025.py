# https://www.janestreet.com/puzzles/robot-baseball-index/

from decimal import *

class PitchState:
    def __init__(self, balls:int, strikes:int, p:Decimal, next_ball_state:'PitchState', next_strike_state:'PitchState'):

        self.balls = balls
        self.strikes = strikes

        # 1. Calculate strategies and probabilities for this pitch state given 
        # p and the potential next states

        if self.balls < 3:
            assert next_ball_state is not None, f"Missing next ball state for PitchState {self.strikes} strikes {self.balls} balls"
        if self.strikes < 2:
            assert next_strike_state is not None, f"Missing next strike state for PitchState {self.strikes} strikes {self.balls} balls"
        # Get probabilities of reaching walk, strikeout and dinger outcomes if 
        # this pitch results in a ball from the previously calculated next ball
        # state if one exists
        p_ball_walk = next_ball_state.p_walk_total if next_ball_state is not None else Decimal(0)
        p_ball_strikeout = next_ball_state.p_strikeout_total if next_ball_state is not None else Decimal(0)
        p_ball_dinger = next_ball_state.p_dinger_total if next_ball_state is not None else Decimal(0)
        # Get probabilities of reaching walk, strikeout and dinger outcomes if 
        # this pitch results in a strike from the previously calculated next 
        # strike state if one exists
        p_strike_walk = next_strike_state.p_walk_total if next_strike_state is not None else Decimal(0)
        p_strike_strikeout = next_strike_state.p_strikeout_total if next_strike_state is not None else Decimal(0)
        p_strike_dinger = next_strike_state.p_dinger_total if next_strike_state is not None else Decimal(0)
        # Calculate the equilibrium strategies for batter and pitcher (which 
        # are equal due to the payout structure of the pitch)
        p_strategy = mixed_strategy(p=p, 
                                    subsequent_strike_payout=4*p_strike_dinger + p_strike_walk, 
                                    subsequent_ball_payout=4*p_ball_dinger + p_ball_walk, 
                                    strikes=self.strikes, 
                                    balls=self.balls)
        # Probability of the pitcher throwing inside the strike zone in the 
        # mixed strategy for this state
        self.p_batter = p_strategy
        # Probability of the batter swinging in the mixed strategy for this 
        # state
        self.p_pitcher = p_strategy

        # 2. Calculate transition probabilities of strikes, balls, dingers in 
        # this state given the mixed strategy

        # Probability this state transitions to the next strike state (or a 
        # strikeout)
        self.p_strike = (1 - p)*self.p_batter*self.p_pitcher + (1 - self.p_batter)*self.p_pitcher + self.p_batter*(1 - self.p_pitcher)
        # Probability this state transitions to the next ball state (or a walk)
        self.p_ball = (1 - self.p_batter)*(1 - self.p_pitcher)
        # Probability this state transitions to a dinger
        self.p_dinger = p*self.p_batter*self.p_pitcher

        # 3. Derive total probabilities for reaching dinger, walk and strikeout 
        # outcomes of the at-bat from this state to calculate the strategies
        # and transition probabilities for preceding states.

        p_walk = Decimal(0)
        p_strikeout = Decimal(0)
        if self.balls == 3:
            p_walk = self.p_ball
        if self.strikes == 2:
            p_strikeout = self.p_strike
        # Total probability of dingers given the strategy of this state and all
        # possible subsequent states
        self.p_dinger_total = self.p_dinger + self.p_strike*p_strike_dinger + self.p_ball*p_ball_dinger
        # Total probability of walks given the strategy of this state and all 
        # possible subsequent states
        self.p_walk_total = p_walk + self.p_strike*p_strike_walk + self.p_ball*p_ball_walk
        # Total probability of strikeouts given the strategy of this state and 
        # all possible subsequent states
        self.p_strikeout_total = p_strikeout + self.p_strike*p_strike_strikeout + self.p_ball*p_ball_strikeout

# Return the optimal mixed strategy for the batter and pitcher as the 
# probability that the batter swings or the pitcher throws inside the strike 
# zone (which are identical given the payout structure for the players)
def mixed_strategy(p:Decimal, subsequent_strike_payout:Decimal, subsequent_ball_payout:Decimal, strikes:int, balls:int):
    strike_payout = subsequent_strike_payout
    ball_payout = Decimal(1) if balls == 3 else subsequent_ball_payout
    swing_strike_payout = 4*p + (1 - p)*strike_payout
    assert swing_strike_payout + ball_payout - 2*strike_payout != 0, "Player strategies do not have a unique equilibrium"
    return (ball_payout - strike_payout) / (swing_strike_payout + ball_payout - 2*strike_payout)

# Recursively determine the optimal mixed strategies for batter and pitcher and
# the state transition probabilities for the given pitch state and probability
# p. The pitch_states dictionary will be populated with the results for this 
# pitch and any possible subsequent pitch states.
def pitch_recursion(balls:int, strikes:int, p:Decimal, pitch_states:dict):
    key = (balls, strikes)
    if key in pitch_states:
        return pitch_states[key]
    state = PitchState(balls=balls, 
                       strikes=strikes, 
                       p=p, 
                       next_ball_state=pitch_recursion(balls + 1, strikes, p, pitch_states) if balls < 3 else None, 
                       next_strike_state=pitch_recursion(balls, strikes + 1, p, pitch_states) if strikes < 2 else None)
    pitch_states[key] = state
    return pitch_states[key]

# Calculate mixed strategies for each pitch state given p, construct a Markov 
# model of at-bats and run it forward to determine the proportion of at-bats 
# reaching a full count
def markov_model(p:Decimal):
    # Calculate mixed strategies and transition probabilities for each pitch 
    # state given p
    pitch_states = {}
    pitch_recursion(balls=0, strikes=0, p=p, pitch_states=pitch_states)
    # Set up Markov model with 12 nodes for pitch states plus strikeout, dinger 
    # and walk outcome nodes. The separate outcome nodes could be combined into
    # one since we don't need to know about the proportion of outcomes 
    # directly, but it doesn't add much overhead.
    pitch_state_to_index = {}
    index_to_pitch_state = []
    for k in pitch_states:
        pitch_state_to_index[k] = len(pitch_state_to_index)
        index_to_pitch_state.append(k)
    strikeout_node_index = len(index_to_pitch_state)
    walk_node_index = strikeout_node_index + 1
    dinger_node_index = walk_node_index + 1
    num_nodes = len(index_to_pitch_state) + 3
    model_node_values = [Decimal(0)] * num_nodes
    # Define the transition matrix for the model
    transitions = []
    for i in range(num_nodes):
        transition_row = [Decimal(0)] * num_nodes
        # Outcome nodes do not transition except to themselves
        if i == strikeout_node_index or i == walk_node_index or i == dinger_node_index:
            transition_row[i] = Decimal(1)
            transitions.append(transition_row)
        else:
            key = index_to_pitch_state[i]
            balls = key[0]
            strikes = key[1]
            # Ball result
            if balls == 3:
                # Next ball transitions to the walk outcome
                transition_row[walk_node_index] += pitch_states[key].p_ball
            else:
                # Transition to next ball state
                transition_row[pitch_state_to_index[(balls + 1, strikes)]] += pitch_states[key].p_ball
            # Strike result
            if strikes == 2:
                # Next strike transitions to a strikeout outcome
                transition_row[strikeout_node_index] += pitch_states[key].p_strike
            else:
                # Transition to next strike state
                transition_row[pitch_state_to_index[(balls, strikes + 1)]] += pitch_states[key].p_strike
            # Dinger result
            transition_row[dinger_node_index] = pitch_states[key].p_dinger
        transitions.append(transition_row)
    # Run the model forward from starting state until completion,
    # keeping a sum of all values that reach the full count node
    p_full_count = Decimal(0)
    model_node_values = [Decimal(0)] * num_nodes
    model_node_values[pitch_state_to_index[(0,0)]] = Decimal(1)
    # An at-bat can last for no more than 6 pitches
    for step in range(7):
        next_node_values = [Decimal(0)] * num_nodes
        for i in range(num_nodes):
            for j in range(num_nodes):
                next_node_values[j] += model_node_values[i] * transitions[i][j]
        p_full_count += next_node_values[pitch_state_to_index[(3,2)]]
        model_node_values = next_node_values
    return p_full_count

def main():
    print("\n######## Jane Street Puzzle - October 2025 ########\n")
    # Search for a p where the sampled slope of dq/dp approaches 0, indicating 
    # peak q. 
    delta = Decimal(0.1**10)
    left = Decimal(0.01)
    right = Decimal(0.99)
    magnitude = Decimal(1.0)
    sampling_rate = 100
    min_p = None
    while magnitude > delta:
        samples = {}
        for i in range(sampling_rate):
            p = ((right - left)*i/sampling_rate) + left
            p_left = p - delta
            p_right = p + delta
            q_left = markov_model(p_left)
            q_right = markov_model(p_right)
            slope = (q_right - q_left)/(p_right - p_left)
            samples[p] = slope
        min_p = None
        min_slope = 0
        for sample_p in samples:
            if min_p is None or abs(samples[sample_p]) < abs(min_slope):
                min_p = sample_p
                min_slope = samples[sample_p]
        magnitude /= 2
        left = min_p - magnitude
        right = min_p + magnitude
        if left <= 0:
            left = Decimal(0.01)
        if right > 1:
            right = Decimal(0.99)
    print(f"Solution: {markov_model(min_p):.10f}")
        
if __name__ == "__main__":
    main()