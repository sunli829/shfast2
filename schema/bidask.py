


bid = 'BID'
ask = 'ASK'
price = 'PRICE'
volume = 'VOLUME'
n = 5
start = 12
prefix = '\t\t'


l1 = [bid, ask]
l2 = [price, volume]
for i in range(0,n):
    for d in l1:
        for t in l2:
            s = f'{prefix}{d}{i+1}_{t} = {start};'
            print(s)
            start += 1

