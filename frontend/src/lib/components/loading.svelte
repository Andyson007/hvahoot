<div class="loading">
  <span></span>
  <span></span>
  <span></span>
</div>

<style>
  @keyframes load {
    from {
      translate: 0 -100%;
    }
    33% {
      translate: 0 0;
    }
    66% {
      translate: 0 0;
    }
    to {
      translate: 0 100%;
    }
  }
  .loading {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    overflow: hidden;

    &>span {
      animation: load 1s infinite ease;
      width: 1rem;
      height: 4rem;
      background-color: var(--main);
      translate: 0 -100%;

      &:nth-child(1) {
        animation-delay: 0s;
      }
      &:nth-child(2) {
        animation-delay: .125s;
      }
      &:nth-child(3) {
        animation-delay: .25s;
      }
    }
  }
</style>