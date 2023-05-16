from functools import reduce
import numpy as np

# data = np.random.randint(0, 2, 11)

# # Master bit 1 = odd number of bits
# # Master bit 0 = even number of bits
# parity = [0, 0, 0, 0, 0]

# bits = [
#   parity[0], parity[1], parity[2], data[0],
#   parity[3], data[1],   data[2],   data[3],
#   parity[4], data[4],   data[5],   data[6],
#   data[7],   data[8],   data[9],   data[10],
# ]

bits = [
  0, 0, 1, 1,
  1, 0, 1, 0,
  0, 0, 1, 0,
  1, 0, 0, 1
]

while True:
  print("\nBits:", bits)
  print("Broken bit:", reduce(lambda x, y: x ^ y, [i for i, bit in enumerate(bits) if bit]))
  bits[10] = 0
  print("New bit:", reduce(lambda x, y: x ^ y, [i for i, bit in enumerate(bits) if bit]))

# Parity bit 1 = odd number of bits
# Parity bit 0 = even number of bits

#0  1  1  0
#1  1  1  0
#1  1  0  1
#1  1  1  0
