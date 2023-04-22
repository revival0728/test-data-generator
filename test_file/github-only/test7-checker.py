import sys
import os

output_dir = os.listdir('./output/')

if len(output_dir) != 10:
    print(output_dir)
    sys.exit(1)

sys.exit(0)