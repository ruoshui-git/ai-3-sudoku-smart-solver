#! /usr/bin/python3
import sys
from pathlib import Path

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
    Path(sys.argv[2]).write_text(','.join([part for part in Path(sys.argv[1]).read_text(
        encoding='utf-8').splitlines()[int(sys.argv[3])].strip().split(',') if part.isnumeric() and int(part) != 0]))

if __name__ == "__main__":
    main()