def main():
    total_included = 0
    total_overlap = 0
    print("Hello")
    with open('input') as f:
        lines = f.readlines()
        for line in lines:
            parts = line.split(',')
            first = parts[0]
            second = parts[1]
            if is_included(first, second):
                total_included += 1
            if overlap(first, second) or overlap(second, first) or is_included(first, second):
                total_overlap += 1
    print(f'Total included={total_included} overlap={total_overlap}')

def is_included(first, second):
    l1 = int(first.split('-')[0])
    r1 = int(first.split('-')[1]) 
    l2 = int(second.split('-')[0])
    r2 = int(second.split('-')[1])
    return (l1 <= l2 and r2 <= r1) or (l2 <= l1 and r1 <= r2)

def overlap(first, second):
    l1 = int(first.split('-')[0])
    r1 = int(first.split('-')[1]) 
    l2 = int(second.split('-')[0])
    r2 = int(second.split('-')[1])
    return (l1 <= l2 and r1 >= l2) or (r1 <= l2 and l1 >= l2 )

if __name__ == '__main__':
    main()
