# Jane Street December 2025 Puzzle

## Description

The [December 2025 puzzle](https://www.janestreet.com/puzzles/robot-javelin-index/) asks about cheating in a robot javelin competition.

Robot javelin is played as a competition between two robots. In each competition, both robots make an initial throw that has a uniformly distributed distance on $[0,1]$. Neither robot knows the other robot's initial throw distance, but can choose to keep their current throw or make a second throw, also uniformly distributed on $[0,1]$. The robot with the greatest final throw distance wins.

In honorable play, the strategies for the robots reach a Nash equilibrium. However, *Spears Robot* has found a way to cheat and can learn whether the opponent's first throw distance was above or below a threshold $d$ which they can select. Presumably $d$ is selected to maximize their probability of winning.

Spears Robot assumes that its cheating is undetected, and all other players are using the initial Nash equilibrium strategy. Having learned about the cheating method it's possible to adjust the strategy of your robot, *Java-lin*, to maximize the chances of winning against Spears Robot. The puzzle asks for the updated probability of Java-lin winning either to 10 decimal places or in exact terms.

## Solution

 The solution is not arrived at with a program and is given entirely below.

## Discussion

### Robot throwing

The mechanics of robot javelin strategies are fairly straightforward. A robot should select some cutoff distance $0 \le c \le 1$, and if the first throw distance is less than $c$, a second throw should be taken in the hopes that it will go further. In a single round of play let's call the first throw distance $t_1$, the second throw distance $t_2$ and the final throwing distance given $c$, $T(c)$, with $0 \le t_1, t_2 \le 1$. 

$$T(c) =  \begin{cases}
               t_2 & \text{if } t_1 < c \\
               t_1 & \text{if } t_1 \ge c
            \end{cases}$$

It would be tempting to assume that $T(c)$ behaves just like a uniformly distributed variable on $[0,1]$, but the option for a second throw is significant. Because the robot will keep $t_1$ if $t_1 > c$, this results in an interesting bimodal distribution given by:

$$p(0 \le T(c) < c) = p(t_1 < c) \cdot p(t_2 < c) = c \cdot c = c^2$$
$$p(c < T(c) \le 1) = p(t_1 < c) \cdot p(t_2 > c) + p(t_1 > c) = c \cdot (1-c) + (1-c) = 1 - c^2$$

If $t_1 < c$ (with probability $c$), a second throw is taken with expected distance $\frac{1}{2}$. If $t_1 \ge c$ (with probability $1-c$), it is kept with expected distance $\frac{c+1}{2}$. Therefore the expected distance of $T(c)$ is:

$$\text{E}[T(c)] = \frac{1}{2} \cdot c + \dfrac{c + 1}{2} \cdot (1-c) = \dfrac{1 + c - c^{2}}{2}$$

### Honorable robot equilibrium strategy

Suppose two players $A$ and $B$ play with strategies $c_A$ and $c_B$, with $c_A \le c_B$. The below discusses the optimal strategy for player $A$ given the strategy of player $B$ to arrive at the equilibrium strategy. The inverse approach isn't given, but would reach the same result.

- If both players rethrow (with probability $c_A \cdot c_B$), the win probability of player $A$ is $\frac{1}{2}$ since both second throws are uniformly distributed on $[0,1]$.

- If player $A$ keeps the first throw and player $B$ rethrows (with probability $(1 - c_A) \cdot c_B$), the win probability of player $A$ is $c_A + \frac{1-c_A}{2}$ since the $A$ throw is uniformly distributed on $[c_A,1]$ and the $B$ throw is uniformly distributed on $[0,1]$.

- If player $A$ rethrows and player $B$ keeps the first throw (with probability $c_A \cdot (1 - c_B)$ ), the win probability of player $A$ is $\frac{1  -c_B}{2}$ since the $A$ throw is uniformly distributed on $[0,1]$ and the $B$ throw is uniformly distributed on $[c_B,1]$.

- If neither player rethrows (with probability $(1 - c_A) \cdot (1 - c_B)$ ), the $A$ throw is uniformly distributed on $[c_A,1]$ and the $B$ throw is uniformly distributed on $[c_B,1]$. Since $c_A < c_B$, the win probabilty of player $A$ is $\frac{1 - c_B}{2}\cdot \frac{1}{1 - c_A}$

For a total probability of $A$ winning:

$$p(A_{\text{win}}) = (c_A \cdot c_B)\dfrac{1}{2} + ((1 - c_A) \cdot c_B) \left(c_A + \dfrac{1-c_A}{2}\right) + (c_A \cdot (1 - c_B))\dfrac{1  -c_B}{2} + ((1 - c_A) \cdot (1 - c_B))\left(\dfrac{1 - c_B}{2}\cdot \dfrac{1}{1 - c_A}\right)$$

or

$$p(A_{\text{win}}) = \dfrac{-c_A^2c_B + c_Ac_B^2 - c_Ac_B + c_A + c_B^2 - c_B + 1}{2}$$

This is quadratic for $c_A$ with respect to $c_B$, and the maximum (where the slope of the function is zero) can be found solving for the roots of the derivative:

$$\dfrac{\partial}{\partial{c_A}} p(A_{\text{win}}) = \dfrac{-c_Ac_B + c_B^2 - c_B + 1}{2}$$

The equilibrium for honorable games will be found at the maximum where $c_A = c_B$, which has roots of the derivative

$$c_A = \dfrac{-1 \pm \sqrt{5}}{2}$$

Since $0 \le c_A \le 1$, the negative root can be ignored and the equilibrium strategy for honorable players is $c = \frac{\sqrt{5} - 1}{2}$. Let's call this value $C$ for brevity.

### Spears Robot's strategy

Spears Robot should select its cheating parameter $d$ equal to the cutoff value for its opponent. This will allow it to determine if a second throw is taken, and adjust its own strategy for taking a second throw  to take advantage of the different distributions. Since Spears Robot assumes its opponent is using the optimal equilibrium strategy, it will select $d = C$. It will then use some strategies $s_1$ and $s_2$, depending on whether it expects its opponent to take one or two throws respectively, in order to maximize its probability of winning in either case.

#### Opponent takes second throw

If the opponents first throw $\text{O}_{t_1}$ is less than $C$, Spears Robot should choose whether or not to take a second throw with the assumption that the opponent will make a second throw with expected distance $\frac{1}{2}$ since it is uniform on $[0,1]$.

Spears robot's probability of winning in this case (which occurs with probability $C$) using strategy $s_2$ is:

$$p(\text{SR}_{\text{win}}| \text{O}_{t_1} < d) = s_2 \cdot \dfrac{1}{2} + (1 - s_2) \cdot \left(s_2 + \dfrac{1 - s_2}{2}\right) = \dfrac{-s_2^2 + s_2 + 1}{2}$$

This is quadratic with a maximum at $\frac{1}{2}$. So $s_2 = \frac{1}{2}$ is the optimal Spears Robot strategy if the opponent rethrows. In this case, Spears Robot's probability of a win is:

$$p(\text{SR}_{\text{win}}| \text{O}_{t_1} < d) = \dfrac{1}{2} \cdot \dfrac{1}{2} + \left(1 - \dfrac{1}{2}\right) \cdot \left(\dfrac{1}{2} + \dfrac{1 - \frac{1}{2}}{2}\right) = \dfrac{5}{8}$$

#### Opponent keeps first throw

If the opponent's first throw is greater than $C$, Spears Robot should assume that the opponent will keep a throw that is uniformly distributed on $[C,1]$.

Spears Robot uses strategy $s_1 \ge C$ in this case. If Spears Robot rethrows (with probability $s_1$) it can only win if the second throw that is uniform on $[0,1]$ is in the region $[C,1]$, with probability $\frac{1}{2}$ and with probability $0$ elsewhere. If it keeps the first throw (with probability $1 - s_1$), uniform on $[s_1,1]$, it wins with probability $1$ if the opponents throw is in the region $[C,s_1]$ and with probability $\frac{1}{2}$ if the opponent's throw is in the region $[s_1,1]$.

$$p(\text{SR}_{\text{win}} | \text{O}_{t_1} > d) = s_1 \cdot \dfrac{1 - C}{2} + (1 - s_1) \cdot \dfrac{1 \cdot (s_1 - C) + \frac{1}{2} \cdot (1 - s_1)}{1 - C}$$

This has a maximum at $s_1 = \frac{5 - \sqrt{5}}{4}$, and a probability of a win in this case of:

$$p(\text{SR}_{\text{win}} | \text{O}_{t_1} > d) =  \dfrac{7}{8} - \dfrac{\sqrt{5}}{5} \approx  0.315983$$

#### Overall win probability

This gives an overall probability the Spears Robot wins against an opponent playing the equilibrium strategy of

$$p(\text{SR}_{\text{win}}) = C \cdot p(\text{SR}_{\text{win}}| \text{O}_{t_1} < d) + (1 - C) \cdot p(\text{SR}_{\text{win}} | \text{O}_{t_1} > d) = \dfrac{3}{2} - \dfrac{17\sqrt{5}}{40} \approx 0.549671$$

Spears Robot's cheating allows it to gain a significant advantage against the optimal play of honorable players.

### Java-lin's strategy

Since Spears Robot will expect to be able to predict whether its opponent takes a second throw or not, Java-lin can use this to its advantage to determine the strategy Spears Robot will use. Spears Robot will expect Java-lin to play using strategy $C$, and will make its own strategy decisions based on whether or not Java-lin's first throw is above or below $C$. Knowing if its first throw was above or below $C$, Java-lin can adjust its own strategy for taking a second throw based on the strategies Spears Robot will take.

#### First throw under $C$

If Java-lin's first throw is less than $C$ (with probability $C$), Spears Robot will use strategy $s_2 = \frac{1}{2}$ as it expects Java-lin to take a second throw. Java-lin's modified strategy $s_2 < j_2 < C$ has win probabilities in individual cases of Spears Robot's first throw:

   - If Spears Robot's first throw is less than $s_2$, Spears robot will rethrow. Java-lin will rethrow if $\text{J}_{t_1}$ is in $[0,j_2]$ and keep the throw if in $[j_2,C]$, resulting in a win probability of:

   $$p(\text{J}_{win} | \text{J}_{t_1} < C, \text{SR}_{t_1} < s_2 ) = \dfrac{1}{C} \cdot \left( j_2 \cdot \dfrac{1}{2} + (C - j_2) \cdot (1 \cdot j_2 + \dfrac{1}{2} \cdot (C - j2))\right)$$

   - If Spears Robot's first throw is greater than $s_2$, Spears robot will keep its first throw. Java-lin will rethrow if $\text{J}_{t_1}$ is in $[0,j_2]$ and keep the throw if in $[j_2,C]$, resulting in a win probability of:

$$p(\text{J}_{win} | \text{J}_{t_1} < C, \text{SR}_{t_1} > s_2 ) = \dfrac{1}{C} \cdot \left(j_2 \cdot \dfrac{1}{2} \cdot (1 - s_2) + (C - j_2) \cdot (1 - p(\text{SR}_{win} | j_2 < \text{J}_{t_1} < C, \text{SR}_{t_1} > s_2 )\right)$$

$$p(\text{J}_{win} | \text{J}_{t_1} < C, \text{SR}_{t_1} > s_2 ) = \dfrac{1}{C} \cdot \left(j_2 \cdot \dfrac{1}{2} \cdot (1 - s_2) + (C - j_2) \cdot \left(1 - \dfrac{\frac{1}{2} \cdot (C - j_2) + 1 \cdot (1 - C)}{1 - s_2}\right) \right) $$

This gives a full equation for the $\text{J}_{t_1} < C$ case as:

$$ p(\text{J}_{win} | \text{J}_{t_1} < C) = s_2 \cdot p(\text{J}_{win} | \text{J}_{t_1} < C, \text{SR}_{t_1} < s_2 ) + (1 - s_2) \cdot p(\text{J}_{win} | \text{J}_{t_1} < C, \text{SR}_{t_1} > s_2 )$$

$$p(\text{J}_{win} | \text{J}_{t_1} < C) = \dfrac{(j_2 \cdot (s_2 - 1)^2 + s_2 \cdot (j_2 + (C - j_2)(C + j_2)) - (C - j_2)*(2s_2 - C - j_2))}{2C}$$

This has a maximum at $j_2 = \frac{7}{12}$, giving a probability of Java-lin winnning in this case:

$$p(\text{J}_{win} | \text{J}_{t_1} < C) = \dfrac{193\sqrt{5} - 287}{384} \approx 0.376461$$

#### First throw over $C$

If Java-lin's first throw $\text{J}_{t_1}$ is greater than $C$ (with probability $1 - C$), Spears Robot will use strategy $s_1 = \frac{5 - \sqrt{5}}{4}$ as it expects Java-lin to keep its first throw.

The optimal strategy for Java-lin if $\text{J}_{t_1} > C$ is to keep the first throw in all circumstances. The probability of Java-lin winning in this case is:

$$p(\text{J}_{win} | \text{J}_{t_1} > C) = s_1 \cdot \left(C \cdot 1 + (1 - C) \cdot \dfrac{1}{2} \right) + (1 - s_1) \cdot \left( (1 - s_1) \cdot \dfrac{1}{2} \cdot \dfrac{1}{1 - C} \right) = \dfrac{1 + 2\sqrt{5}}{8} \approx 0.684016$$


### Probability of Java-lin winning

The final probability of Java-lin winning against Spears Robot given the derived strategies is:

$$p(\text{J}_{\text{win}}) = C \cdot \dfrac{193\sqrt{5} - 287}{384} + (1 - C) \cdot \dfrac{1 + 2\sqrt{5}}{8} = \dfrac{229}{192} - \dfrac{5\sqrt{5}}{16} \approx 0.4939370904$$