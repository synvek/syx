-- SYX Neovim LSP thin client (W926 / ADR-1270 / C-07 first-slice).
-- Speaks the same `syx lsp` stdio server as editors/vscode-syx.

local M = {}

local function default_cmd()
  -- Prefer PATH; callers may override with setup({ cmd = { … } }).
  return { "syx", "lsp" }
end

function M.setup(opts)
  opts = opts or {}
  local cmd = opts.cmd or default_cmd()

  vim.filetype.add({
    extension = { sy = "syx" },
  })

  vim.api.nvim_create_autocmd("FileType", {
    pattern = "syx",
    callback = function(ev)
      vim.lsp.start({
        name = "syx",
        cmd = cmd,
        root_dir = vim.fs.root(ev.buf, { "syx.toml", ".git" }) or vim.fn.getcwd(),
        on_attach = opts.on_attach,
      })
    end,
  })
end

return M
