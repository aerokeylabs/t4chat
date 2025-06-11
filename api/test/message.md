Here's the Python code to invert a binary search tree (also known as mirroring the tree). This operation swaps the left and right children for every node in the tree.

```python
class TreeNode:
  def __init__(self, val=0, left=None, right=None):
    self.val = val
    self.left = left
    self.right = right

def invertTree(root: TreeNode) -> TreeNode:
  """
  Inverts a binary tree.

  Args:
    root: The root of the binary tree.

  Returns:
    The root of the inverted binary tree.
  """
  if not root:
    return None

  # Swap the left and right children
  root.left, root.right = root.right, root.left

  # Recursively invert the left and right subtrees
  invertTree(root.left)
  invertTree(root.right)

  return root

# --- Example Usage ---

def print_tree_inorder(root):
  """Helper function to print the tree in-order for verification."""
  if root:
    print_tree_inorder(root.left)
    print(root.val, end=" ")
    print_tree_inorder(root.right)

if __name__ == "__main__":
  # Create an example BST
  #       4
  #      / \
  #     2   7
  #    / \ / \
  #   1  3 6  9
  root = TreeNode(4)
  root.left = TreeNode(2)
  root.right = TreeNode(7)
  root.left.left = TreeNode(1)
  root.left.right = TreeNode(3)
  root.right.left = TreeNode(6)
  root.right.right = TreeNode(9)

  print("Original tree (in-order traversal):")
  print_tree_inorder(root)
  print("\n")

  inverted_root = invertTree(root)

  print("Inverted tree (in-order traversal):")
  print_tree_inorder(inverted_root)
  print("\n")

  # Expected output for inverted tree (in-order traversal):
  #       4
  #      / \
  #     7   2
  #    / \ / \
  #   9  6 3  1
  # Should print: 9 7 6 4 3 2 1
```

### Explanation:

1.  **`TreeNode` Class:**

- This class defines a node in the binary tree. Each node has a `val` (value), a `left` child, and a `right` child.

2.  **`invertTree(root: TreeNode) -> TreeNode` Function:**

- **Base Case:** `if not root: return None`
  - If the `root` is `None` (an empty tree or we've reached past a leaf node), there's nothing to invert, so we return `None`.
- **Swap Children:** `root.left, root.right = root.right, root.left`
  - This is the core of the inversion. It simultaneously swaps the left and right children of the current node. Python's tuple assignment makes this clean and efficient.
- **Recursive Calls:**
  - `invertTree(root.left)`: We recursively call `invertTree` on the _new_ left child (which was the original right child).
  - `invertTree(root.right)`: We recursively call `invertTree` on the _new_ right child (which was the original left child).
  - The recursion ensures that this swapping process happens for every node down to the leaves.
- **Return Root:** `return root`
  - After processing the current node and its subtrees, we return the (now inverted) `root` of the subtree we were processing.

### How it works (Recursive Approach):

The function uses a classic recursive approach. Imagine you're at a specific node:

1.  **Do the work for the current node:** Swap its left and right children.
2.  **Recursively conquer the subproblems:** Call the same function on the (now swapped) left child and the (now swapped) right child.

This ensures that every node in the tree eventually has its children swapped, resulting in a completely inverted tree.

### Time and Space Complexity:

- **Time Complexity:** O(N), where N is the number of nodes in the binary tree.
  - Each node in the tree is visited exactly once to perform the swap operation.
- **Space Complexity:** O(H), where H is the height of the binary tree.
  - This is due to the recursion stack. In the worst case (a skewed tree), H can be N, leading to O(N) space. In the best case (a balanced tree), H is (log N), leading to O(log N) space.
