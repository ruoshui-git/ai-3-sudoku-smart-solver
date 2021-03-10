#! /usr/bin/python3
# from . import _rust
from .rust import run

# def main():
#     with open(sys.argv[1], 'r') as inf, open(sys.argv[2], 'w') as outf:
#         lineno = int(sys.argv[3])

#         parts = inf.readlines()[lineno].strip().split(',')
#         end = []
#         for part in parts:
#             if part.isnumeric():
#                 a = int(part)
#                 if a != 0:
#                     end.append(str(a))

#         outf.write(','.join(end))

def main():
    run()

if __name__ == "__main__":
    main()