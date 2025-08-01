# https://www.janestreet.com/puzzles/robot-road-trip-index/

from decimal import *

# Derivative of the expected cost of miles lost due to overtaking given a, 
# computed using SymPy's integration solver. Returns a high precision Decimal
def derivative(a):    
    #-3*a**2*ln(a) + 2*a**2/3 + a**2*ln(2) - a*(-12*a*ln(a) - 12*a)/6 - 4*a*ln(a) - 4*a + 4*a*ln(2) - 5/2 + 1/(3*a)
    return Decimal(-3)*(a**Decimal(2))*(a.ln()) + Decimal(2)*a**(Decimal(2))/Decimal(3) + a**Decimal(2)*Decimal(2).ln() - a*(-Decimal(12)*a*a.ln() - Decimal(12)*a)/Decimal(6) - Decimal(4)*a*a.ln() - Decimal(4)*a + Decimal(4)*a*Decimal(2).ln() - Decimal(5)/Decimal(2) + Decimal(1)/(Decimal(3)*a)

def main():
    print("\n######## Jane Street Puzzle - July 2025 ########\n")
    getcontext().prec = 20
    # Iterating over small increments of a, find the zero crossing of the 
    # derivative of the expected cost of overtaking
    scale = 100000000000
    best_a = 0
    best_delta = 1
    # Due to the amount of time it takes to iterate over `a` between 1 and 2 
    # in increments of 1/10^11, the starting value has been chosen to be close 
    # to the experimentally determined answer to reduce execution time
    max_a = Decimal(2.0)
    start_a = Decimal(1.17714)
    for x in range(scale):
        a = start_a + Decimal(x)/Decimal(scale)
        der = derivative(a)
        delta = abs(der)
        if delta < best_delta:
            best_a = a
        if der > 0:            
            break
        if a > max_a:
            break
    print("Solution: {:.10f}".format(best_a))    

if __name__ == "__main__":
    main()