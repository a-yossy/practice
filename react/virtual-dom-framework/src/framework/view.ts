/** Nodeが取りうる3種の型 */
type NodeType = VNode | string | number;

/** 属性の型 */
type Attributes = { [key: string]: string | Function };

const isVNode = (node: NodeType): node is VNode => {
  return typeof node !== 'string' && typeof node !== 'number';
}

export interface View<State, Actions> {
  (state: State, actions: Actions): VNode;
}

/**
 * 仮想DOM
 */
export interface VNode {
  nodeName: keyof HTMLElementTagNameMap;
  attributes: Attributes;
  children: NodeType[];
}

/**
 * 仮想DOMを作成する
 */
export function h(
  nodeName: keyof HTMLElementTagNameMap,
  attributes: Attributes,
  ...children: NodeType[]
): VNode {
  return { nodeName, attributes, children };
}

/**
 * リアルDOMを生成する
 */
export function createElement(node: NodeType): HTMLElement | Text {
  if (!isVNode(node)) {
    return document.createTextNode(node.toString());
  }

  const el = document.createElement(node.nodeName);
  setAttributes(el, node.attributes);
  node.children.forEach(child => el.appendChild(createElement(child)));

  return el;
}

/**
 * targetに属性を設定する
 */
function setAttributes(target: HTMLElement, attrs: Attributes): void {
  for (let attr in attrs) {
    if (isEventAttr(attr)) {
      const eventName = attr.slice(2);
      target.addEventListener(eventName, attrs[attr] as EventListener);
    } else {
      target.setAttribute(attr, attrs[attr] as string);
    }
  }
}

function isEventAttr(attr: string): boolean {
  // onから始まる属性名はイベントとして扱う
  return /^on/.test(attr);
}

enum ChangedType {
  /** 差分なし */
  None,
  /** nodeの型が違う */
  Type,
  /** テキストノードが違う */
  Text,
  /** ノード名(タグ名)が違う */
  Node,
  /** inputのvalueが違う */
  Value,
  /** 属性が違う */
  Attr
}

/**
 * 受け取った2つの仮想DOMの差分を検知する
 */
function hasChanged(a: NodeType, b: NodeType): ChangedType {
  // different type
  if (typeof a !== typeof b) {
    return ChangedType.Type;
  }

  // different string
  if (!isVNode(a) && a !== b) {
    return ChangedType.Text;
  }

  // 簡易的比較()
  if (isVNode(a) && isVNode(b)) {
    if (a.nodeName !== b.nodeName) {
      return ChangedType.Node;
    }
    if (a.attributes.value !== b.attributes.value) {
      return ChangedType.Value;
    }
    if (JSON.stringify(a.attributes) !== JSON.stringify(b.attributes)) {
      return ChangedType.Attr;
    }
  }

  return ChangedType.None;
}

/**
 * 仮想DOMの差分を検知し、リアルDOMに反映する
 */
export function updateElement(
  parent: HTMLElement,
  oldNode: NodeType,
  newNode: NodeType,
  index = 0
): void {
  // oldNodeがない場合は新しいノード
  if (!oldNode) {
    parent.appendChild(createElement(newNode));
    return;
  }

  const target = parent.childNodes[index];

  // newNodeがない場合はそのノードを削除する
  if (!newNode) {
    parent.removeChild(target);
    return;
  }

  // 両方ある場合は差分を検知し、パッチ処理を行う
  const changeType = hasChanged(oldNode, newNode);
  switch (changeType) {
    case ChangedType.Type:
    case ChangedType.Text:
    case ChangedType.Node:
      parent.replaceChild(createElement(newNode), target);
      return;
    case ChangedType.Value:
      // valueの変更時にNodeを置き換えてしまうとフォーカスが外れてしまうため
      updateValue(
        target as HTMLInputElement,
        (newNode as VNode).attributes.value as string
      )
      return;
    case ChangedType.Attr:
      // 属性の変更は、Nodeを再作成する必要がないので更新するだけ
      updateAttributes(
        target as HTMLElement,
        (oldNode as VNode).attributes,
        (newNode as VNode).attributes
      )
      return;
  }

  //　再帰的にupdateElementを呼び出し、childrenの更新処理を行う
  if (isVNode(oldNode) && isVNode(newNode)) {
    for (
      let i = 0;
      i < newNode.children.length || i < oldNode.children.length;
      i++
    ) {
      updateElement(
        target as HTMLElement,
        oldNode.children[i],
        newNode.children[i],
        i
      );
    }
  }
}


// NodeをReplaceしてしまうとinputのフォーカスが外れてしまうため
function updateAttributes(
  target: HTMLElement,
  oldAttrs: Attributes,
  newAttrs: Attributes
): void {
  // remove attrs
  for (let attr in oldAttrs) {
    if (!isEventAttr(attr)) {
      target.removeAttribute(attr);
    }
  }
  // set attrs
  for (let attr in newAttrs) {
    if (!isEventAttr(attr)) {
      target.setAttribute(attr, newAttrs[attr] as string);
    }
  }
}

// updateAttributesでやりたかったけど、value属性としては動かないので別途作成
function updateValue(target: HTMLInputElement, newValue: string): void {
  target.value = newValue;
}
