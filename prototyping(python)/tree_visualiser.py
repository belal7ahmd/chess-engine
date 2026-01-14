from PIL import Image, ImageDraw, ImageFont
import chess


class TreeNode:
    def __init__(self, board: chess.Board, value=None):
        self.board = board # Example: store state
        self.value = value
        self.children = []

    def is_leaf(self):
        return len(self.children) == 0

def draw_tree(node, x, y, x_offset, draw, font):
    # Draw children first to get lines behind nodes
    if not node.is_leaf():
        child_x = x - (len(node.children) - 1) * x_offset / 2
        for child in node.children:
            # Draw line from parent to child
            draw.line((x, y, child_x, y + 100), fill="black", width=2)
            # Recurse
            draw_tree(child, child_x, y + 100, x_offset / 2, draw, font)
            child_x += x_offset

    # Draw the current node (circle)
    radius = 20
    draw.ellipse((x - radius, y - radius, x + radius, y + radius), fill="skyblue", outline="black")
    
    # Draw node value
    label = str(node.value) if node.value is not None else "?"
    draw.text((x - 10, y - 10), label, fill="black", font=font)


def generate_moves_tree(board: chess.Board, depth: int) -> TreeNode:
    root = TreeNode(board)
    __create_moves_tree__(root, depth)
    return root
    
def __create_moves_tree__(node: TreeNode, depth: int):
    if depth == 0:
        return node
        
    for move in node.board.legal_moves:
        print(f"Generating move: {move}")
        new_board = node.board.copy()
        new_board.push(move)
        child_node = TreeNode(new_board)
        child_node.value = node.board.san(move)  # Store move UCI as value for visualization
        node.children.append(child_node)
        __create_moves_tree__(child_node, depth - 1)

    return node

# Initialize Image
img_width, img_height = 400, 400
img = Image.new('RGB', (img_width, img_height), 'white')
draw = ImageDraw.Draw(img)
font = ImageFont.load_default()

# Example Usage
root = TreeNode(None, "Root")
child1 = TreeNode(None, "A")
child2 = TreeNode(None, "B")
root.children = [child1, child2]
child1.children = [TreeNode(None, "C"), TreeNode(None, "D")]
child2.children = [TreeNode(None, "E"), TreeNode(None, "F")]

#depth = 2

#root = generate_moves_tree(chess.Board(), depth)  # Replace None with actual engine instance

# Draw starting from the top-center
draw_tree(root, img_width // 2, 50, 100, draw, font)
img.show()