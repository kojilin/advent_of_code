package solutions.year_2020;

import java.util.HashMap;
import java.util.Map;

public class Day23 {
    static class Node {
        long value;
        Node next;
        Node previous;

        public Node(long value) {
            this.value = value;
        }
    }

    public static void main(String[] args) {
        final Map<Long, Node> map = new HashMap<>();
        long[] input = new long[] { 3, 6, 8, 1, 9, 5, 7, 4, 2 };
        //long[] input = new long[] { 3, 8, 9, 1, 2, 5, 4, 6, 7 };
        Node prev = null;
        for (int i = 0; i < input.length; i++) {
            long value = input[i];
            Node currentNode = new Node(value);
            map.put(value, currentNode);
            if (prev != null) {
                currentNode.previous = prev;
                prev.next = currentNode;
            }
            prev = currentNode;
        }
        for (int i = 10; i <= 1_000_000; i++) {
            Node currentNode = new Node(i);
            map.put((long) i, currentNode);
            if (prev != null) {
                currentNode.previous = prev;
                prev.next = currentNode;
            }
            prev = currentNode;
        }
        map.get(input[0]).previous = prev;
        prev.next = map.get(input[0]);

        //first
        Node current = map.get(input[0]);
        for (int i = 0; i < 10_000_000; i++) {
            Node fourth = current.next.next.next.next;
            current.next.next.next.next = null;
            Node picked = current.next;
            picked.previous = null;

            current.next = fourth;
            fourth.previous = current;

            long destination = current.value - 1;
            if (destination <= 0) {
                destination = 1_000_000;
            }

            while (true) {
                boolean existingInPicked = false;
                Node check = picked;
                while (check != null) {
                    if (check.value == destination) {
                        destination -= 1;
                        if (destination <= 0) {
                            destination = 1_000_000;
                        }
                        existingInPicked = true;
                        break;
                    }
                    check = check.next;
                }
                if (!existingInPicked) {
                    break;
                }
            }
            Node destinationNode = map.get(destination);
            Node temp = destinationNode.next;
            destinationNode.next = picked;
            picked.previous = destinationNode;
            picked.next.next.next = temp;
            temp.previous = picked;

            current = current.next;
        }
        Node first = map.get(1L).next;
        System.out.println(first.value * first.next.value);
    }
}
