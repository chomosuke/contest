def find_one_common(s1, s2):
    d = {}
    for i in range(len(s1)):
        d[s1[i]] = i
    for i in range(len(s2)):
        if d.get(s2[i]):
            return (s2[i], d[s2[i]], i)
    return None

def find_prefix_and_suffix(s1, s2):
    prefix = []
    suffix = []

    for i in range(min(len(s1), len(s2))):
        if s1[i] == s2[i]:
            prefix.append(s1[i])
        else:
            s1 = s1[i:]
            s2 = s2[i:]
            break

    for i in range(min(len(s1), len(s2))):
        if s1[-1 - i] == s2[-1 - i]:
            suffix.insert(0, (s1[-1 - i]))
        else:
            s1 = s1[:len(s1) - i]
            s2 = s2[:len(s2) - i]
            break
    return (prefix, suffix, s1, s2)

def git_diff(s1, s2):
    prefix, suffix, mid1, mid2 = find_prefix_and_suffix(s1, s2)
    if len(mid1) == 0 or len(mid2) == 0:
        for i in prefix:
            print(i, end=" ")
        for i in ["+" + i for i in mid1]:
            print(i, end=" ")
        for i in ["-" + i for i in mid2]:
            print(i, end=" ")
        for i in suffix:
            print(i, end=" ")
        return
    else:
        r = find_one_common(mid1, mid2)
        if r is None:
            for i in prefix:
                print(i, end=" ")
            for i in ["+" + i for i in mid1]:
                print(i, end=" ")
            for i in ["-" + i for i in mid2]:
                print(i, end=" ")
            for i in suffix:
                print(i, end=" ")
                return
        else:
            for i in prefix:
                print(i, end=" ")
            c, i1, i2 = r
            git_diff(mid1[:i1], mid2[:i1])

            print(c, end=" ")

            git_diff(mid1[i1+1:], mid2[i2+1:])

            for i in suffix:
                print(i, end=" ")

            return


a = ["x", "y", "a", "d", "0", "1", "2"]
b = ["x", "y", "c", "d", "9", "1", "2"]
print(git_diff(a, b))
