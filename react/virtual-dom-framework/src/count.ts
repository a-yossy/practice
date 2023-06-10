import { View, h } from './framework/view.js';
import { App } from './framework/controller.js';
import { ActionTree } from './framework/action.js';

type State = typeof state;
type Actions = typeof actions;

const state = {
  count: 0
};

const actions: ActionTree<State> = {
  increment: (state: State) => {
    state.count++;
  }
}

const view: View<State, Actions> = (state, actions) => {
  return h(
    'div',
    null,
    h('p', null, state.count),
    h(
      'button',
      { type: 'button', onclick: () => actions.increment(state) },
      'count up'
    )
  )
}

new App<State, Actions>({
  el: '#app',
  state,
  view,
  actions
});
