<script>
	import { ripple } from "$lib/ripple";
</script>

<style>
  .top {
    position: sticky;
    top: 0;
    z-index: var(--z-top);
    height: var(--top-height);
    padding: 0.5rem 1rem 0 1rem;
    display: flex;
    flex-direction: row;
    background: #f4f4f4df;
    backdrop-filter: blur(2px);
  }

  .box {
    position: relative;
    flex: 1;
    border-radius: 0.5rem;
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 5%);
    z-index: calc(var(--z-top) + 1);
    background: #fff;
    display: flex;
    flex-direction: row;
  }

  .account {
    align-self: center;
    margin-inline-start: auto;
    display: flex;
    flex-direction: row;
  }
  
  .names {
    display: flex;
    flex-direction: column;
    text-align: right;
    align-items: flex-end;
    margin-inline-end: 1rem;
    margin-top: 0.25rem;
  }


  .user-name {
    font-weight: 600;
    font-size: 1rem;
    max-width: 12rem;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .account-name {
    font-size: 0.9rem;
    color: #999;
    max-width: 12rem;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-top: 0.125rem;
  }

  .pic {
    width: 2.75rem;
    height: 2.75rem;
    border-radius: 50%;
    background: var(--red);
    color: #fff;
    font-weight: 600;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.35rem;
    user-select: none;
    cursor: pointer;
    box-shadow: 0 4px 8px 0 rgba(0,0,0,.12),0 2px 4px 0 rgba(0,0,0,.08);
    margin-inline-end: 1rem;
  }
</style>

<div class="top">
  <div class="box">
    <div class="account">
      <div class="names">
        <div class="user-name">Ramiro Aisen</div>
        <div class="account-name">Openradio</div>
      </div>
      <div class="pic ripple-container" use:ripple>R</div>
    </div>
  </div>
</div>