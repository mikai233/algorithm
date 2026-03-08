class ListNode(
    var `val`: Int,
) {
    var next: ListNode? = null
}

class Solution {
    fun deleteNode(node: ListNode?) {
        val nextNode = node?.next ?: return
        node.`val` = nextNode.`val`
        node.next = nextNode.next
    }
}
