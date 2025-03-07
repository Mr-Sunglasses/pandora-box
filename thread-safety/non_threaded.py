import threading
import time
import gzip
import io

n: int = 0

def f():
    global n
    for i in range(1, 100000000+1):
        n+=1


def download_and_write():
    import urllib.request
    
    # time.sleep(10)
    data = urllib.request.urlopen("https://www.python.org").read()
    data = gzip.decompress(data).decode("utf-8")
    with open("main.txt", "w") as file:
        file.write(data)


# t1 = threading.Thread(target=f)
# t2 = threading.Thread(target=download_and_write)

# t1.start()
# t2.start()

# t1.join()
# t2.join()

f()
download_and_write()

print("The value of n is: %d", n)

