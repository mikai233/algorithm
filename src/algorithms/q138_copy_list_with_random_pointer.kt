class Node(
    var `val`: Int,
) {
    var next: Node? = null
    var random: Node? = null
}

class Solution {
    fun copyRandomList(node: Node?): Node? {
        var curr = node
        while (curr != null) {
            val copy = Node(curr.`val`)
            val next = curr.next
            curr.next = copy
            copy.next = next
            curr = next
        }
        curr = node
        while (curr != null) {
            curr.next?.random = curr.random?.next
            curr = curr.next?.next
        }
        curr = node
        val dummy = Node(-1)
        dummy.next = node
        var currCopy: Node? = dummy
        while (curr != null) {
            val copy = curr.next
            curr.next = copy?.next
            currCopy?.next = copy
            currCopy = currCopy?.next
            curr = curr.next
        }
        return dummy.next
    }
}
