import threading

n: int = 0

def f():
    global n
    for i in range(1, 1000000000+1):
        n = n + 1

#f()
#f()
t1 = threading.Thread(target=f)
t2 = threading.Thread(target=f)

t1.start()
t2.start()

t1.join()
t2.join()

print(f"The value of n is {n}")
