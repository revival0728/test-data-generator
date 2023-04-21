import sys, errno

try:
    N = int(input())
    for _ in range(N):
        print(input()[::-1])
except IOError as e:
    if e.errno == errno.EPIPE:
        sys.exit(0)
