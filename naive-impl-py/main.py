from dataclasses import dataclass

from typing import TypeVar

T = TypeVar("T")


@dataclass
class LinkedList:
    head: "Node" = None

    def push(self, value: T):
        if self.head:
            node = Node(value, self.head)
            self.head = node
        else:
            node = Node(value)
            self.head = node

    def pop(self):
        if self.head:
            value = self.head.elem
            self.head = self.head.next
            return value
        else:
            return None

    def iter(self):
        cur_node = self.head
        while cur_node:
            yield cur_node.elem
            cur_node = cur_node.next


@dataclass
class Node:
    elem: T
    next: "Node" = None


ll = LinkedList()
print(ll)

for i in range(5):
    ll.push(i)
    print(f"pushing {i} | {ll}")

print("=" * 60)
print("iterating over ll")
for n in ll.iter():
    print(n)
print("=" * 60)

for _ in range(5):
    print(f"popping {ll.pop()} | {ll}")
