function SkeletonDashboard() {
  return (
    <div className="skeleton-dashboard">
      <div className="skeleton-header">
        <div className="skeleton-block skeleton-title" />
        <div className="skeleton-header-stats">
          <div className="skeleton-block skeleton-stat" />
          <div className="skeleton-block skeleton-stat" />
          <div className="skeleton-block skeleton-stat" />
          <div className="skeleton-block skeleton-stat" />
        </div>
      </div>

      <div className="skeleton-block skeleton-heatmap" />

      <div className="skeleton-cards">
        <div className="skeleton-block skeleton-card" />
        <div className="skeleton-block skeleton-card" />
        <div className="skeleton-block skeleton-card" />
        <div className="skeleton-block skeleton-card" />
      </div>

      <div className="skeleton-block skeleton-table" />
    </div>
  );
}

export default SkeletonDashboard;
