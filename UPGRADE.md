# 🔥 Yew 0.21 Upgrade Plan 🔥

*Upgrading from Yew 0.19 to 0.21 for maximum vibec0re performance* ✨

## 📋 Current State Analysis

### Dependencies to Update
- `yew`: `0.19` → `0.21`
- `yew-router`: `0.16` → `0.18`
- `gloo`: `0.8` → `0.11`
- `gloo-*` crates: Various minor/major version bumps

### Major Breaking Changes Identified

1. **Hook System Changes**
   - Hook positioning rules became stricter
   - All hooks must be at function component top-level
   - No conditional hook calls allowed

2. **Router API Changes**
   - `use_history` hook removed/renamed
   - History management API restructured
   - Route matching syntax updates

3. **Event Handler Changes**
   - Event callback type system updated
   - Some event types may have changed signatures

4. **Component API Updates**
   - Function component macro changes
   - Props system updates
   - Context API modifications

## 🎯 Upgrade Strategy

### Phase 1: Dependency Updates
- [ ] Update `Cargo.toml` with new versions
- [ ] Run `cargo update` to pull latest compatible versions
- [ ] Document initial compilation errors

### Phase 2: Hook System Fixes
- [ ] **Authentication Component** (`src/app/authenticate.rs`)
  - Fix hook positioning violations (lines 57-59, 80-81)
  - Move all `use_input_ref` calls to component top-level
  - Restructure conditional logic to avoid hook positioning issues

- [ ] **Router Hooks** (Multiple files)
  - Replace `use_history` with new router API
  - Update imports in:
    - `src/app/page.rs`
    - `src/app/authenticate.rs` 
    - `src/app/router.rs`
    - `src/app/backends/page.rs`
    - `src/app/backends/list.rs`
    - `src/app/frontends/list.rs`
    - `src/app/frontends/page.rs`
    - `src/app/nav.rs`

### Phase 3: Custom Hook System Updates
- [ ] **Client Hooks** (`src/api/client.rs`)
  - Update `use_client()` to implement new `Hook` trait
  - Fix `client::State<T>` to work with new hook system
  - Update `use_frontends()` and `use_frontend()` hooks
  - Ensure all custom hooks return `BoxedHook<'_, T>`

### Phase 4: Event Handler Updates
- [ ] **Form Components** (`src/app/frontends/form.rs`)
  - Fix `Callback<FocusEvent>` vs `SubmitEvent` type mismatch
  - Update event handler signatures to match new API
  - Review all form event bindings

### Phase 5: Component System Updates
- [ ] **Function Components**
  - Update component macro usage if needed
  - Check for props system changes
  - Validate context provider/consumer patterns

### Phase 6: Router System Overhaul
- [ ] **Route Definitions** (`src/app/router.rs`)
  - Update route syntax if changed
  - Fix `use_route` hook usage
  - Update route matching patterns

- [ ] **Navigation Components** (`src/app/nav.rs`)
  - Update link generation
  - Fix navigation state management

### Phase 7: Gloo Updates
- [ ] **Network Layer** (`gloo-net` 0.2 → 0.6)
  - Update HTTP client usage
  - Check for API changes in request/response handling
  - Update error handling patterns

- [ ] **Storage & Console** 
  - Update `gloo-storage` usage patterns
  - Verify `gloo-console` compatibility

## 🚨 High-Risk Areas

### 1. Authentication Flow
- Heavy use of conditional hooks
- Complex state management with router integration
- JWT token handling with navigation

### 2. Form Components
- Complex event handling
- Multiple hook interactions
- State synchronization issues

### 3. Custom Hook System
- Non-standard hook implementations
- State management patterns
- API client integration

## 🧪 Testing Strategy

### Pre-Upgrade Testing
- [ ] Document current functionality
- [ ] Create comprehensive test cases
- [ ] Screenshot all UI states

### Post-Upgrade Testing
- [ ] Verify all routes work
- [ ] Test authentication flow end-to-end
- [ ] Validate form submissions
- [ ] Check frontend/backend management
- [ ] Verify dark theme styling

## 📝 Implementation Notes

### Development Approach
1. **Branch Strategy**: Create `yew-0.21-upgrade` branch
2. **Incremental Updates**: Fix one component at a time
3. **Compilation Driven**: Let compiler errors guide fixes
4. **Testing After Each Phase**: Ensure no regressions

### Potential Challenges
- **Hook Positioning**: May require significant refactoring
- **Router Migration**: Could need component restructuring  
- **Custom Hooks**: May need complete rewrite
- **Event Handlers**: Type system changes could be extensive

### Fallback Plan
- Keep current 0.19 version as backup
- Document all breaking changes encountered
- Consider gradual migration over multiple PRs

## 🎉 Expected Benefits Post-Upgrade

- **Performance**: Latest Yew optimizations
- **Security**: Updated dependencies with patches
- **Features**: Access to newest Yew capabilities
- **Ecosystem**: Better compatibility with latest crates
- **Future-Proofing**: Easier future upgrades

---

*Ready to vibe this upgrade to the next level!* 🦀⚡✨