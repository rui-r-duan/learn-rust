#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

typedef struct Node {
    int data;
    struct Node* next;
} Node;

typedef struct List {
    Node* head;
} List;

List* ListNew() {
    List* lst = malloc(sizeof(List));
    lst->head = NULL;
    return lst;
}

void ListPush(List* lst, int elem) {
    if (lst == NULL) {
        return;
    }
    Node* new_node = malloc(sizeof(Node));
    if (new_node == NULL) {
        return;
    }
    new_node->data = elem;
    new_node->next = lst->head;
    lst->head = new_node;
}

Node* ListPop(List* lst) {
    if (lst == NULL) {
        return NULL;
    }
    Node* p = lst->head;
    if (p == NULL) {
        return NULL;
    }
    else {
        lst->head = p->next;
        return p;
    }
}

Node* ListPeek(List* lst) {
    if (lst == NULL) {
        return NULL;
    }
    return lst->head;
}

void ListDelete(List* lst) {
    if (lst == NULL) {
        return;
    }
    Node* p = lst->head;
    while (p != NULL) {
        Node* n = p->next;
        free(p);
        p = n;
    }
    free(lst);
}

void printLinkedlist(List* lst) {
    if (lst == NULL) {
        return;
    }
    Node* p = lst->head;
    while (p != NULL) {
        printf("%d ", p->data);
        p = p->next;
    }
    printf("\n");
}

int main() {
    List* lst = ListNew();
    Node* p = ListPop(lst);
    assert(p == NULL);

    ListPush(lst, 1);
    ListPush(lst, 2);
    ListPush(lst, 3);
    printLinkedlist(lst);

    p = ListPop(lst);
    assert(p->data == 3);
    free(p);
    printLinkedlist(lst);

    p = ListPop(lst);
    assert(p->data == 2);
    free(p);
    printLinkedlist(lst);

    p = ListPeek(lst);
    assert(p->data == 1);
    p = ListPeek(lst);
    assert(p->data == 1);
    p->data = 100;
    assert(p->data == 100);
    printLinkedlist(lst);

    p = ListPop(lst);
    printLinkedlist(lst);

    ListDelete(lst);
    lst = NULL;
    printLinkedlist(lst);
    return 0;
}
